use rand::Rng; // Import the Rng trait for random number generation
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
enum GameError {
    EncounteredError,
}

type GameResult = Result<(), GameError>;


pub async fn level4() {
    println!("Welcome to the Dark Mines of Errors!");
    sleep(Duration::from_millis(2000)).await;

    let mut rng = rand::thread_rng();

    let total_moves = 5;
    println!("You will get a random number. If the number is odd, you will step safely; if not, you will encounter an error.");
    sleep(Duration::from_millis(2000)).await;
    println!("You have a total of 5 moves. If you survive all of them, you win!");

    let mut successful_moves = 0;

    for move_number in 1..=total_moves {
        println!("Move {} of {}", move_number, total_moves);
        let random_number = rng.gen_range(0..127);

        match process_move(random_number).await {
            Ok(_) => {
                println!("Safe step! Random number: {}", random_number);
                successful_moves += 1;
            }
            Err(err) => {
                println!(
                    "Oh no! You encountered an error. Random number: {}",
                    random_number
                );
                handle_error(err).await;
                break;
            }
        }

        sleep(Duration::from_millis(1000)).await;
    }

    if successful_moves == total_moves {
        println!("Congratulations! You survived all 5 moves and won the game!");
    } else {
        println!("Game over! Better luck next time.");
    }
}

async fn process_move(random_number: u32) -> GameResult {
    if random_number % 2 != 0 {
        Ok(())
    } else {
        Err(GameError::EncounteredError)
    }
}

async fn handle_error(error: GameError) {
    match error {
        GameError::EncounteredError => {
            println!("Error encountered: A mine has exploded! Be careful next time.");
            sleep(Duration::from_millis(2000)).await;
        }
    }
}
