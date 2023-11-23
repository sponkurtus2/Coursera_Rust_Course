use rand::Rng;
use std::io;

// Library to make request
use reqwest::*;

mod practice_activity_1;
mod max_crate;


// Define player's Structure
struct Player {
    user_name: String,
    score: u32
}

// Define a trait so we can use it with our player
trait Printable {
    fn to_string(&self) -> String;
}

// Generic function to collect Player data
fn collect_input<T: std::str::FromStr>(prompt: &str) -> T {
    // We'll loop until the user input is correct
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse() {
            Ok(value) => {return value}
            // If the user input is incorrect, with the continue word, we tell the program to loop again
            Err(_) => continue,
        }
    }
}

// Function to get players
fn collect_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let mut num_players: u32 = 0;

    loop {
        num_players = collect_input::<u32>("Hoy many players?");
        if num_players <= 0 {
            println!("Error, 0 players not available...");
            continue;
        } else {
            break;
        }
    }

    for i in 1..num_players {
          let user_name = collect_input(format!("Player {} name: ", i).as_str());
          players.push(Player { user_name, score: 0 });
      }
      players

}

// Function to get the max number of players
fn create_max_range(players: &Vec<Player>) -> u32 {
    players.len() as u32 * 50
}

// Function to generate a random number
/* fn generate_number(max_range: u32) -> u32 {
    rand::thread_rng().gen_range(1..max_range)
}
*/

// Function to generate a random number using an API
#[tokio::main]
async fn generate_number(max_range: u32) -> Result<u32> {
    let body = reqwest::get(
        "https://www.random.org/integers/?num=1&min=1&max={MAX}\
        &col=1&base=10&format=plain&rnd=new".replace("{MAX}", &max_range.to_string()),
    )
        .await?
        .text()
        .await?;

    let value = body.trim().parse::<u32>().expect("Error in parsing");
    println!("Value: {}", value);
    Ok(value)
}

fn main() {
    // practice_activity_1::practice_activity_1();
    //max_crate::max_function();
    let _ = generate_number(20);
}
