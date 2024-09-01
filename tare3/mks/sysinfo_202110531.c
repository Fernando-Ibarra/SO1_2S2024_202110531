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