
use tokio::time::{sleep, Duration};

pub async fn level3() {
    let mountains = vec![
        ("Everest", 5),
        ("K2", 7),
        ("Denali", 4),
        ("Makalu", 6),
        ("Lhotse", 3),
    ];
    let mut stamina = 20;
    let mut memory: Vec<&str> = Vec::new();

    println!("Welcome to the Extended Mountain Memory Game!");
    println!("Your starting stamina: {}", stamina);

    // Explore the mountains
    for &(mountain, difficulty) in &mountains {
        if stamina < difficulty {
            println!("Not enough stamina to climb {}. Resting to recover stamina...", mountain);
            stamina += 5; // Stamina recovery
            println!("Recovered stamina! Current stamina: {}", stamina);
        }
        if stamina >= difficulty {
            stamina -= difficulty;
            memory.push(mountain);
            println!("You explored {}! Remaining stamina: {}", mountain, stamina);
            sleep(Duration::from_secs(1)).await;
        } else {
            println!("Still not enough stamina to climb {}. Game over!", mountain);
            return;
        }
    }

    println!("\nGreat job exploring all the mountains!");
    println!("But wait... Bonus mountains have appeared!");

    let bonus_mountains = vec![
        ("Kangchenjunga", 8),
        ("Annapurna", 6),
    ];

    for &(bonus_mountain, bonus_difficulty) in &bonus_mountains {
        if stamina < bonus_difficulty {
            println!("You don't have enough stamina for {}. Skipping...", bonus_mountain);
            continue;
        }
        stamina -= bonus_difficulty;
        memory.push(bonus_mountain);
        println!( "You climbed the bonus mountain: {}! Remaining stamina: {}",bonus_mountain, stamina);
        sleep(Duration::from_secs(1)).await;
    }

    println!("\nYou've reached the final stage: Revisiting all the mountains in reverse order!");

    // Revisit mountains in reverse order
    while let Some(last_mountain) = memory.pop() {
        println!("Revisiting {}...", last_mountain);
        sleep(Duration::from_secs(1)).await;
    }

    println!("\nCongratulations! You've completed the Mountain Memory Game!");
}


