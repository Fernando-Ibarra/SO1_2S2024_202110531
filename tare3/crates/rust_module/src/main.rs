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
    // make ps aux | grep "project.sh" and delete the bash process
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