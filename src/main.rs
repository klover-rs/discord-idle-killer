mod asar;
mod check_installed_clients;
mod inject;
mod eject;

use std::io;

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
                println!("No non-injected clients found");
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
                println!("You chose: \n\n{}\n{}\n{}", selected_client.basename, selected_client.path, selected_client.injected);

                inject::inject(&selected_client.basename).await.unwrap();
            } else {
                println!("Invalid choice");
            }
        }
        2 => {
            let injected_clients: Vec<&check_installed_clients::ClientInfo> = result.iter()
            .filter(|client| client.injected)
            .collect();

            if injected_clients.is_empty() {
                println!("No injected clients found");
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
                println!("You chose: \n\n{}\n{}\n{}", selected_client.basename, selected_client.path, selected_client.injected);

                eject::eject(&selected_client.basename).unwrap();
            } else {
                println!("Invalid choice");
            }

        }
        _ => {
            println!("Invalid choice");
        }
    }

}
