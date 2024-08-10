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
MODULE_DESCRIPTION("Module that reads the ram usage of the system");
MODULE_VERSION("1.0");

#define PROC_NAME "ram_usage"

static int systeminfo_show(struct seq_file *m, void *v) {
    struct sysinfo i;
    struct task_struct *task;
    si_meminfo(&i);
    seq_printf(m, "{\n");
    seq_printf(m, "\t\"total_ram\": %lu,\n", i.totalram * 4);
    seq_printf(m, "\t\"free_ram\": %lu,\n", i.freeram * 4);
    seq_printf(m, "\t\"processes\": [\n");
    for_each_process(task) {
        seq_printf(m, "\t\t{\n");
        seq_printf(m, "\t\t\t\"pid\": %d,\n", task->pid);
        seq_printf(m, "\t\t\t\"name\": \"%s\",\n", task->comm);
        seq_printf(m, "\t\t\t\"children\": [\n");
        struct task_struct *child;
        list_for_each_entry(child, &task->children, sibling) {
            seq_printf(m, "\t\t\t\t{\n");
            seq_printf(m, "\t\t\t\t\t\"pid\": %d,\n", child->pid);
            seq_printf(m, "\t\t\t\t\t\"name\": \"%s\"\n", child->comm);
            seq_printf(m, "\t\t\t\t},\n");
        }
        seq_printf(m, "\t\t\t]\n");
        seq_printf(m, "\t\t},\n");
    }
    seq_printf(m, "\t]\n");
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