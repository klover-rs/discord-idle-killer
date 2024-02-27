mod asar;
mod check_installed_clients;
mod inject;
mod eject;

use std::io;
use std::process::Command;

const ANSI_RED: &str = "\x1b[31m";
const ANSI_RESET: &str = "\x1b[0m";

fn kill_discord(which_discord: &str) {
    let is_windows = cfg!(target_os = "windows");

    if is_windows {
        Command::new("powershell")
            .arg("-Command")
            .arg(&format!("Stop-Process -Name {}", which_discord))
            .output()
            .expect("Failed to kill the process");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&format!("kill -9 $(ps aux | grep {} | awk '{{print $2}}')", which_discord))
            .output()
            .expect("Failed to kill the process");
    }

    println!("Process {} killed successfully", which_discord);
}


#[tokio::main]
async fn main() {

    let mut input = String::new();

    println!("do you want to inject or eject?\n1. inject\n2. eject");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let choice: usize = input.trim().parse().expect("Please enter a number");

    let result: Vec<check_installed_clients::ClientInfo> = check_installed_clients::check_installed_clients();

    match choice {
        1 => {

            let non_injected_clients: Vec<&check_installed_clients::ClientInfo> = result.iter()
            .filter(|client| !client.injected)
            .collect();

            if non_injected_clients.is_empty() {
                println!("{}No non-injected clients found{}", ANSI_RED, ANSI_RESET);
                return;
            }
    
            for (index, client) in non_injected_clients.iter().enumerate() {
                println!("{}: {}", index + 1, client.basename);
            }

            println!("Enter the number of the client you want to inject:");

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let choice: usize = input.trim().parse().expect("Please enter a number");

            if choice > 0 && choice <= non_injected_clients.len() {
                let selected_client = non_injected_clients[choice - 1];
               
                kill_discord(&selected_client.basename);

                inject::inject(&selected_client.basename).await.unwrap();
            } else {
                println!("{}Invalid choice{}", ANSI_RED, ANSI_RESET);
            }
        }
        2 => {
            let injected_clients: Vec<&check_installed_clients::ClientInfo> = result.iter()
            .filter(|client| client.injected)
            .collect();

            if injected_clients.is_empty() {
                println!("{}No injected clients found{}", ANSI_RED, ANSI_RESET);
                return;
            }

            for (index, client) in injected_clients.iter().enumerate() {
                println!("{}: {}", index + 1, client.basename);
            }

            println!("Enter the number of the client you want to inject:");

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let choice: usize = input.trim().parse().expect("Please enter a number");

            if choice > 0 && choice <= injected_clients.len() {
                let selected_client = injected_clients[choice - 1];
                kill_discord(&selected_client.basename);
                eject::eject(&selected_client.basename).unwrap();
            } else {
                println!("{}Invalid choice{}", ANSI_RED, ANSI_RESET);
            }

        }
        _ => {
            println!("{}Invalid choice{}", ANSI_RED, ANSI_RESET);
        }
    }

}
