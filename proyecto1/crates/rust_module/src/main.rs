use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::thread::sleep;
use serde::{Deserialize, Serialize};
use std::thread;
use reqwest::Client;
use chrono::Local;

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
    
    // Create a logs container (python server)
    let _log_container_id: String = build_container();

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
        println!("..................... Eliminando Contenedores .....................");
        println!("-----> Ctrl+C para salir");

        // Analize containers status and delete them
        let _ = read_containers(_log_container_id.clone()).await;
        
        // If you want to do something in each loop iteration, you can do it here
        println!("Contenedores eliminados, preparando la siguiente eliminación...");
        
        // Sleep for 10 seconds
        sleep(Duration::from_secs(30));
    }

    // delete cron job
    delete_cron_job();

    let _ = make_graphs().await;

    sleep(Duration::from_secs(10));

    let _ = make_graphs_process().await;

    sleep(Duration::from_secs(30));

    // delete log container
    let output = Command::new("sudo")
        .args(&["docker", "rm", "-f", &_log_container_id])
        .output()
        .expect("Failed to execute docker rm command");
    if output.status.success() {
        println!("Log container deleted successfully");
    } else {
        eprintln!("Failed to delete log container");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Code to execute after loop is cancelled
    println!("Realizando limpieza antes de salir...");

}

fn build_container() -> String {
    let output = Command::new("sudo")
        .args(&["docker", "compose", "-f", "./crates/rust_module/server/docker-compose.yml", "build", "--no-cache"])
        .output()
        .expect("Failed to execute docker compose command");

    if output.status.success() {
        let output = Command::new("sudo")
            .args(&["docker", "compose", "-f", "./crates/rust_module/server/docker-compose.yml", "up", "-d"])
            .output()
            .expect("Failed to execute docker compose command");
        if output.status.success() {
            println!("Docker containers built and started successfully");
            let ps_output = Command::new("sudo")
                .args(&["docker", "ps", "-q", "--filter", "ancestor=server-backend"])
                .output()
                .expect("Failed to execute docker ps command");

            if ps_output.status.success() {
                let container_id = String::from_utf8_lossy(&ps_output.stdout);
                let log_container_id = container_id.trim().to_string();
                println!("Container ID: {}", container_id.trim());
                return log_container_id;
            } else {
                eprintln!("Failed to obtain container ID");
                return String::from("");
            }
        } else {
            eprintln!("Failed to start Docker containers");
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
            return String::from("");
        }        
    } else {
        eprintln!("Failed to build and start Docker containers");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return String::from("");
    }
}

async fn read_containers(logs_container_id: String) {
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

        // containers_list = cpu.processes;
        containers_list.retain(|container| container.id_container[..12] != logs_container_id);

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
        let my_cpu_log = CPUSERVER {
            total_ram: cpu.total_ram,
            free_ram: cpu.free_ram,
            ram_in_use: cpu.ram_in_use,
            processes: containers_list_v2,
            time: Local::now().format("%H:%M:%S").to_string(),
        };

        let _ = fetch_data(my_cpu_log).await;

        let containers_list = Arc::new(Mutex::new(containers_list));

        /* 
            delete de the containers in the middle of the list 
            The first 3 and the last 2 containers alive
        */
         if n > 5 {
            let mut i = 0;
            while i < n {
                if i > 2 && i < n - 2 {
                    let containers_list = Arc::clone(&containers_list);
                    thread::spawn(move || {
                        let containers_list = containers_list.lock().unwrap();
                        let output = Command::new("sudo")
                            .args(&["docker", "rm", "-f", &containers_list[i].id_container])
                            .output()
                            .expect("Failed to execute docker rm command");
                        if output.status.success() {
                            println!("Container {} deleted successfully", containers_list[i].id_container);
                        } else {
                            eprintln!("Failed to delete container");
                            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                        }
                    });
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

async fn fetch_data(payload: CPUSERVER) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let response = client.post("http://localhost:8000/read-module")
        .json(&payload)
        .send()
        .await?
        .text()
        .await?;

    println!("Response: {}", response);

    Ok(())
}

async fn make_graphs() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let response = client.post("http://0.0.0.0:8000/make_graphs")
        .send()
        .await?
        .text()
        .await?;

    println!("Response: {}", response);

    Ok(())
}

async fn make_graphs_process() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let response = client.post("http://0.0.0.0:8000/make_process_graphs")
        .send()
        .await?
        .text()
        .await?;

    println!("Response: {}", response);

    Ok(())
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