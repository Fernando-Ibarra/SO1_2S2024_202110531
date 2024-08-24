use std::process::Command;

fn main() {
    let output = Command::new("sudo")
        .args(&["docker", "compose", "-f", "./server/docker-compose.yml", "up", "--build", "-d"])
        .output()
        .expect("Failed to execute docker compose command");
    
    if output.status.success() {
        println!("Docker containers built and started successfully");
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Failed to build and start Docker containers");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
