use std::time::Duration;
use tokio::time::sleep;

pub async fn level2() {
    println!("welcome to Loops of the River");
    println!("loading");
    sleep(Duration::from_millis(3000)).await;
    println!("\n\nTask 1: Check whether the water in the river is deep enough to make a boat float\n");

    let mut water_level = 10;
    let max_safe_level = 3;
    while water_level > max_safe_level {
        sleep(Duration::from_millis(1000)).await;
        println!("Your boat is moving forward. Please keep checking water level. Current water level: {}", water_level);
        
        // Nested loop to simulate additional checks during movement
        for i in 1..=3 {
            println!("  Performing safety check {}...", i);
            sleep(Duration::from_millis(500)).await;
        }

        water_level -= 1;
    }
    println!("Warning! Max safe water level reached. You can't boat now.");
    sleep(Duration::from_millis(1000)).await;

    println!("\nTask 2: Counting the number of fishes in the river\n");
    sleep(Duration::from_millis(1000)).await;
    println!("There are infinite number of fishes in the river, but we will stop counting at 5.\n");

    let mut count = 0;
    loop {
        println!("Counting the number of fishes. Current count is: {}", count);
        sleep(Duration::from_millis(1000)).await;

        // Nested loop to simulate size checks while counting
        for size_check in 1..=2 {
            println!("  Checking size of fish {}...", size_check);
            sleep(Duration::from_millis(500)).await;
        }

        if count == 5 {
            println!("5 fishes counted. Taking a break from the loop.\n");
            break;
        }
        count += 1;
    }

    sleep(Duration::from_millis(1000)).await;
    println!("\nTask 3: Catch the fish of a specified size (4)\n");

    let size = 10;
    let mut found_count = 0;

    for fish_size in 0..size {
        // Nested loop to simulate some extra steps for each fish
        for step in 1..=2 {
            println!("  Step {} for processing fish size {}...", step, fish_size);
            sleep(Duration::from_millis(500)).await;
        }

        if fish_size < 4 {
            println!("Fish size {} is too small. Skipping.\n", fish_size);
            continue;
        }

        found_count += 1;
        println!("Fish of specified size found. Current count: {}", found_count);
    }

    println!("\nTotal fish of specified size: {}", found_count);
}
