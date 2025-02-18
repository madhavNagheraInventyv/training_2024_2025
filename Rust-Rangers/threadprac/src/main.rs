use std::{sync::{Arc, RwLock}, thread, process};
use chrono::{DateTime, Duration, Local};
use rand::Rng;

#[derive(Debug, Clone)]
struct User {
    id: i32,
    timestamp: DateTime<Local>,
}

fn main() {
    let user_vec: Arc<RwLock<Vec<User>>> = Arc::new(RwLock::new(Vec::new()));

    let clone1 = Arc::clone(&user_vec);
    let thread1 = thread::spawn(move || {
        loop {
            {
                let mut rng = rand::thread_rng();
                let user = User {
                    id: rng.gen_range(1..200), 
                    timestamp: Local::now(),
                };
                let mut vec = clone1.write().unwrap();
                vec.push(user);
                println!("User added, total users: {}", vec.len());
            }
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    let clone2 = Arc::clone(&user_vec);
    let thread2 = thread::spawn(move || {
        loop {
            {
                let vec = clone2.read().unwrap();
                println!("Total users: {}", vec.len());
            }
            thread::sleep(std::time::Duration::from_secs(2));
        }
    });

    let clone3 = Arc::clone(&user_vec);
    let thread3 = thread::spawn(move || {
        loop {
            {
                let current_time = Local::now();
                let mut vec = clone3.write().unwrap();
                let before_len = vec.len();
                vec.retain(|user| current_time.signed_duration_since(user.timestamp) < Duration::seconds(7));
                let after_len = vec.len();
                println!("Deleted {} old users", before_len - after_len);
            }
            thread::sleep(std::time::Duration::from_secs(3)); 
        }
    });

    let clone4 = Arc::clone(&user_vec);
    let thread4 = thread::spawn(move || {
        loop {
            {
                let vec = clone4.read().unwrap();
                println!("User List: {:?}", vec);
            }
            thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    let clone5 = Arc::clone(&user_vec);
    let thread5 = thread::spawn(move || {
        loop {
            {
                let vec = clone5.read().unwrap();
                if vec.len() >= 13{
                    println!("13 users created, exiting...");
                    process::exit(1);
                }
            }
            thread::sleep(std::time::Duration::from_secs(5)); 
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();
    thread5.join().unwrap();
}
