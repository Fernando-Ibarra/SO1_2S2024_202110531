use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::thread::sleep;
use serde::{Deserialize, Serialize};
// use reqwest::Client;
// use serde_json::json;
// use std::collections::HashMap;
// use anyhow::{Result, anyhow};

#[derive(Deserialize, Serialize, Debug)]
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

fn main()  {
    
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
        read_containers(_log_container_id.clone());
        
        // If you want to do something in each loop iteration, you can do it here
        println!("Contenedores eliminados, preparando la siguiente eliminación...");
        
        // Sleep for 10 seconds
        sleep(Duration::from_secs(10));
    }

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

    // TODO: LAST FETCH TO SERVE TO MAKE GRAPHICS

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

// -> Result<(), anyhow::Error>
fn read_containers(logs_container_id: String) {
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
        // println!("Processes: {:?}", cpu.proccess);

        // vector to store the containers
        let mut containers_list: Vec<DockerContainerProccess> = Vec::new();

        containers_list = cpu.processes;

        // delete logs_container_id of the list of containers
        containers_list.retain(|container| container.id_container != logs_container_id);

        // bubble sort algorithm to sort the containers by cpu usage
        let n = containers_list.len();
        for i in 0..n {
            for j in 0..n-i-1 {
                if containers_list[j].cpu_usage < containers_list[j+1].cpu_usage {
                    containers_list.swap(j, j+1);
                }
            }
        }

        // print the containers sorted by cpu usage
        for container in containers_list.iter() {
            println!("Container: {}", container.name);
            println!("CPU Usage: {}%", container.cpu_usage);
            println!("Command Line: {}", container.command_line);
            println!("RSS: {} KB", container.rss);
            println!("VSZ: {} KB", container.vsz);
            println!("Memory Usage: {:.2}%", container.mem_usage);
            println!("---------------------------------");
        }

        // TODO: ANALIZE THE CONTAINERS AND DELETE THE ONES THAT ARE IN THE MIDDLE OF THE LIST

        // TODO: FETCH TO THE SERVER THE SURVIVE CONTAINERS AND RAM USAGE 

        
        // println!("Containers: {}", containers);

        // let containers: HashMap<String, String> = serde_json::from_str(&containers).unwrap();
        // println!("Containers: {:?}", containers);
        // let client = reqwest::Client::new();
        // let res = client.post("http://localhost:8000/read-module")
        //     .json(&containers)
        //     .send()
        //     .await?;
        // println!("Response: {:?}", res);
        // return Ok(());
    } else {
        eprintln!("Failed to read containers");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        // return only an error
        // return Err(anyhow!("Failed to read containers"));
    }
}



/*  
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

*/