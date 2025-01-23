// use tokio::time::{sleep, Duration};
use std::time::Duration;
use tokio::time::sleep;

 //#[tokio::main]
pub async fn level_1() {
    println!("finding the creature");
    sleep(Duration::from_millis(3000)).await;

    const INITIAL_ENERGY: i32 = 50; // Immutable variable for initial energy
    const CREATURE_NAME: &str = "dianasour"; // Immutable variable for the creature's name
    let mut phoenix_energy = INITIAL_ENERGY;
    let is_daytime = true;

    println!("A wild {CREATURE_NAME} has appeared!");
    sleep(Duration::from_millis(3000)).await;

    println!(
        "{CREATURE_NAME} gains or loses energy depending on the time of day. At daytime, it regains energy to grow stronger, while at night, it loses energy."
    );
    sleep(Duration::from_millis(3000)).await;

    println!("Initial energy of {CREATURE_NAME}: {}", phoenix_energy);
    sleep(Duration::from_millis(3000)).await;

    println!(
        "The current energy level allows {CREATURE_NAME} to cover {} sq. km in flight.",
        flight_area(&mut phoenix_energy)
    );
    sleep(Duration::from_millis(3000)).await;

    match is_daytime {
        true => {
            println!("It's daytime! {CREATURE_NAME} is absorbing the sun's energy!");
            regain_energy(&mut phoenix_energy);
        }
        false => {
            println!("It's nighttime. {CREATURE_NAME} is losing energy.");
            lose_energy(&mut phoenix_energy);
        }
    }
    sleep(Duration::from_millis(1000)).await;

    println!("New energy level of {CREATURE_NAME}: {}", phoenix_energy);

    match phoenix_energy {
        101.. => {
            println!(
                "{CREATURE_NAME} has reached maximum energy and is preparing to transform into an ultimate monster form!"
            );
        }
        0..=9 => {
            println!(
                "{CREATURE_NAME} is critically low on energy and is at risk of fading away."
            );
        }
        _ => {
            println!(
                "{CREATURE_NAME} maintains a stable energy level, continuing its journey."
            );
        }
    }
    
   super::level_2::level2().await;
}




 fn regain_energy(energy: &mut i32) {
    *energy += 20;
}

 fn lose_energy(energy: &mut i32) {
    *energy -= 15;
}

 fn flight_area(energy: &mut i32) -> f32 {
    (*energy as f32) * 0.75
}
