use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use serde::{Deserialize, Serialize};
use std::io;
use reqwest;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    location: String,
    bio: String,
    r#type: String,
    followers: i32,
    following: i32,
    login: String,
    name: String,
}

fn main() {
    println!("Input your Github username to get your stats.");
    let username = get_username();
    match get_user_data(username, "alsonick") {
        Ok(res) => res,
        Err(err) => println!("Error: {}", err)
    };
    
}

fn get_username() -> String {
    println!("GitHub Username:");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Operation failed.");
    return username;
}

#[tokio::main]
async fn get_user_data(username: String, user_agent_constant: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/users/{}", username);
    let client = reqwest::Client::new();
    let response = client.get(url).header(reqwest::header::USER_AGENT, user_agent_constant)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<User>().await {
                Ok(parsed) => {
                    println!("Hey there, {}!", parsed.name);
                    let table = vec![
                        vec!["Name".cell(), parsed.name.cell().justify(Justify::Right)],
                        vec!["Username".cell(), parsed.login.cell().justify(Justify::Right)],
                        vec!["Bio".cell(), parsed.bio.cell().justify(Justify::Right)],
                        vec!["Type".cell(), parsed.r#type.cell().justify(Justify::Right)],
                        vec!["Followers".cell(), parsed.followers.cell().justify(Justify::Right)],
                        vec!["Following".cell(), parsed.following.cell().justify(Justify::Right)],
                    ]
                    .table()
                        .table()
                        .title(vec![
                            "Keys".cell().bold(true),
                            "Values".cell().bold(true),
                        ])
                        .bold(true);
                    
                    assert!(print_stdout(table).is_ok());
                },
                Err(err) => println!("Error: {}", err)
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("You're not authorized to make this request.");
        },
        error => {
            panic!("Something went wrong: {}", error)
        }
    }
    Ok(())
}