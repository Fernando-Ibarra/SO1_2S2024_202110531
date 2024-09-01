# Tarea 3

### C module
```c
#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/mm.h>
#include <linux/sched.h>
#include <linux/time.h>
#include <linux/jiffies.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("FernandoIbarra");
MODULE_DESCRIPTION("Module that reads the ram usage of the system and identifies the docker process");
MODULE_VERSION("1.0");

#define CMDLINE_LEN 256
#define PROC_NAME "sysinfo_202110531"

static int systeminfo_show(struct seq_file *m, void *v) {
    struct sysinfo i;
    struct task_struct *task;
    char *string1 = "containerd-shim";
    int first_process = 1;
    int first_docker = 1;

    si_meminfo(&i);
    seq_printf(m, "{\n");
    seq_printf(m, "\t\"total_ram\": %lu,\n", i.totalram * 4);
    seq_printf(m, "\t\"free_ram\": %lu,\n", i.freeram * 4);
    seq_printf(m, "\t\"ram_in_use\": %lu,\n", (i.totalram - i.freeram) * 4);
    seq_printf(m, "\t\"processes\": [\n");
    for_each_process(task) {
    
        char *string2 = task->comm;
        if ( strstr(string1,string2) != NULL ) {
            if (first_docker == 1) {
                first_docker = 0;
                continue;
            }
            
            if (!first_process) {
                seq_printf(m, ",\n");
            }
            first_process = 0;
            
            seq_printf(m, "\t\t{\n");
            seq_printf(m, "\t\t\t\"pid\": %d,\n", task->pid);
            seq_printf(m, "\t\t\t\"name\": \"%s\",\n", task->comm);
            unsigned long total_jiffies = jiffies;
            unsigned long total_time = task->utime + task->stime;
            unsigned long cpu_usage = ((total_time*10000)/total_jiffies);
            seq_printf(m, "\t\t\t\"cpu_usage\": %lu,\n", cpu_usage);

            struct mm_struct *mm = task->mm;
            if (mm) {
                // Start and end of the command line arguments
                unsigned long arg_start = mm->arg_start;
                unsigned long arg_end = mm->arg_end;

                // Size of the command line arguments
                unsigned long len = arg_end - arg_start;

                // Buffer for the command line
                char *command_line = kmalloc(len + 1, GFP_KERNEL);
                if (command_line) {
                    memset(command_line, 0, len + 1);

                    // Read the command line arguments
                    if (access_process_vm(task, arg_start, command_line, len, 0) > 0) {
                        // Replace null characters with spaces
                        for (unsigned long i = 0; i < len; i++) {
                            if (command_line[i] == '\0') {
                                command_line[i] = ' ';  // Replace null characters with spaces
                            }
                        }
                        seq_printf(m, "\t\t\t\"command_line\": \"%s\",\n", command_line);

                        // Get the container ID
                        char *id_ptr = strstr(command_line, "-id ");
                        if (id_ptr) {
                            id_ptr += 4; 
                            char *id_end = strpbrk(id_ptr, " \0");
                            if (id_end) {
                                *id_end = '\0';
                            }
                            seq_printf(m, "\t\t\t\"id_container\": \"%s\",\n", id_ptr);
                        }
                    }

                    // Free the buffer
                    kfree(command_line);
                } else {
                    seq_printf(m, "\"command_line\": \"<memory allocation failed>\",\n");
                }

                unsigned long rss = get_mm_rss(mm) * PAGE_SIZE;
                seq_printf(m, "\t\t\t\"rss\": %lu,\n", rss / 1024);
                seq_printf(m, "\t\t\t\"vsz\": %lu,\n", mm->total_vm * PAGE_SIZE / 1024);

                // Memory usage in percentage
                unsigned long total_memory = i.totalram * i.mem_unit;
                unsigned long mem_usage = ((100000 * rss) / total_memory)/ 100;
                // transform mem_usage (07) to string and add a dot before the last character (0.7)
                char mem_usage_str[10];
                sprintf(mem_usage_str, "%lu.%lu", mem_usage / 100, mem_usage % 100);
                seq_printf(m, "\t\t\t\"mem_usage\": %s\n", mem_usage_str);               
            }
            seq_printf(m, "\t\t}");
            // seq_printf(m, "\t\t},\n");
        } else {
            continue;
        }
    }
    // Remove the last comma from the last process

    seq_printf(m, "\n\t]\n");
    seq_printf(m, "}\n");

    return 0;
}

static int systeminfo_open(struct inode *inode, struct file *file) {
    return single_open(file, systeminfo_show, NULL);
}

static const struct proc_ops systeminfo_ops = {
    .proc_open = systeminfo_open,
    .proc_read = seq_read,
};

static int __init systeminfo_init(void) {
    proc_create(PROC_NAME, 0, NULL, &systeminfo_ops);
    printk(KERN_INFO "Module loaded successfully\n");
    return 0;
}

static void __exit systeminfo_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "Module removed successfully\n");
}

module_init(systeminfo_init);
module_exit(systeminfo_exit);
```


### Rust 
```rust
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::thread::sleep;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DockerContainerProccess {
    pid: i64,
    name: String,
    cpu_usage: i64,
    command_line: String,
    id_container: String,
    rss: i64,
    vsz: i64,
    mem_usage: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct CPU {
    total_ram: i64,
    free_ram: i64,
    ram_in_use: i64,
    processes: Vec<DockerContainerProccess>,
}

#[derive(Deserialize, Serialize, Debug)]
struct CPUSERVER {
    total_ram: i64,
    free_ram: i64,
    ram_in_use: i64,
    processes: Vec<DockerContainerProccess>,
    time: String,
}

#[tokio::main]
async fn main() {
    // Create a shared atomic boolean to control the main loop
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Handler Ctrl+C signal
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\nCtrl+C detected! Preparing to exit...");
    }).expect("Error setting Ctrl-C handler");

    // Main loop
    while running.load(Ordering::SeqCst) {
        println!("..................... Contenedores .....................");
        println!("-----> Ctrl+C para salir");

        
        let _ = read_containers().await;
                
        // Sleep for 10 seconds
        sleep(Duration::from_secs(30));
    }

    // delete cron job
    delete_cron_job();

    sleep(Duration::from_secs(30));

    // Code to execute after loop is cancelled
    println!("Realizando limpieza antes de salir...");

}

async fn read_containers() {
    // read containers using cat /proc/sysinfo_202110531
    let output = Command::new("cat")
        .args(&["/proc/sysinfo_202110531"])
        .output()
        .expect("Failed to execute cat command");
    if output.status.success() {
        // parse the output of the command
        let json_data = String::from_utf8_lossy(&output.stdout);

        // Deserialize the JSON data into the CPU struct
        let cpu: CPU = serde_json::from_str(&json_data)
            .expect("Failed to deserialize JSON data into CPU struct");

        // Use the data
        println!("Total RAM: {}", cpu.total_ram);
        println!("Free RAM: {}", cpu.free_ram);

        // vector to store the containers
        let mut containers_list: Vec<DockerContainerProccess> = cpu.processes;

        // bubble sort algorithm to sort the containers by cpu usage and rss and vsz
        let n = containers_list.len();
        for i in 0..n {
            for j in 0..n-i-1 {
                if containers_list[j].cpu_usage < containers_list[j+1].cpu_usage {
                    containers_list.swap(j, j+1);
                }
            }
        }

        for i in 0..n {
            for j in 0..n-i-1 {
                if containers_list[j].cpu_usage == containers_list[j+1].cpu_usage {
                    if containers_list[j].rss < containers_list[j+1].rss {
                        containers_list.swap(j, j+1);
                    }
                }
            }
        }

        for i in 0..n {
            for j in 0..n-i-1 {
                if containers_list[j].cpu_usage == containers_list[j+1].cpu_usage {
                    if containers_list[j].rss == containers_list[j+1].rss {
                        if containers_list[j].vsz < containers_list[j+1].vsz {
                            containers_list.swap(j, j+1);
                        }
                    }
                }
            }
        }

        // clone of the containers list
        let containers_list_v2 = containers_list.clone();
        let containers_list_v3 = containers_list.clone();

        let containers_list = Arc::new(Mutex::new(containers_list));

        /* 
            delete de the containers in the middle of the list 
            The first 3 and the last 2 containers alive
        */
         if n > 5 {
            let mut i = 0;
            while i < n {
                if i > 2 && i < n - 2 {
                    i += 1;
                } else {
                    if i == 0 {
                        println!("High Utilization Container");
                    }
                    if i == (n - 2) {
                        println!("Low Utilization Container");
                    }
                    println!("Container {} is alive", &containers_list_v3[i].id_container);
                    i += 1;
                }
            }
        }

    } else {
        eprintln!("Failed to read containers");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}


fn delete_cron_job() {
    let output = Command::new("ps")
        .args(&["aux"])
        .output()
        .expect("Failed to execute ps aux command");
    if output.status.success() {
        let ps_output = String::from_utf8_lossy(&output.stdout);
        let ps_output = ps_output.split("\n");
        for line in ps_output {
            if line.contains("project.sh") {
                let line = line.split_whitespace().collect::<Vec<&str>>();
                let pid = line[1];
                let output = Command::new("kill")
                    .args(&["-9", pid])
                    .output()
                    .expect("Failed to execute kill command");
                if output.status.success() {
                    println!("Cron job deleted successfully");
                } else {
                    eprintln!("Failed to delete cron job");
                    eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
    } else {
        eprintln!("Failed to read processes");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
```


### Images
![image](/img/ContainersT3.png)
