//Macros
// use lazy_static::lazy_static;
// lazy_static! {
//     static ref CONFIG: String = String::from("Configuration Loaded");
// }
// fn main() {
//     println!("{}", *CONFIG);  // Dereferences the reference to the actual value
// }                             










// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref BUFFER: Mutex<Vec<u8>> = Mutex::new((0..255).collect());
// }

// fn main() {
//     // Ensure BUFFER is initialized
//     lazy_static::initialize(&BUFFER);     //Forces the initialization of the lazy static variable BUFFER immediately.

//     // Work with the initialized data
//     work_with_initialized_data();    
// }

// fn work_with_initialized_data() {
//     let buffer = BUFFER.lock().unwrap(); // Lock the mutex before accessing
//     println!("First bytes of BUFFER: {:?}", &buffer[..100]);
// }





// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref COUNTER: Mutex<i32> = Mutex::new(0);
// }

// fn initialize_counter(value: i32) {
//     let mut counter = COUNTER.lock().unwrap(); // Lock the Mutex
//     *counter = value;                          // Dereference to modify the value
//     println!("Counter initialized to {}", *counter);
// }

// fn main() {
//     initialize_counter(10); // Initialize counter with 10
// }





// //traits
// use lazy_static::lazy_static;
// use std::collections::HashMap;

// // Define the LazyStatic trait
// pub trait LazyStatic {
//     fn init();                     //The init() method allows you to specify the initialization logic for the resource or value.
// }

// // Global configuration stored in a HashMap
// lazy_static! {
//     static ref CONFIG: HashMap<&'static str, &'static str> = {
//         let mut m = HashMap::new();
//         m.insert("server_host", "127.0.0.1");
//         m.insert("server_port", "8080");
//         m
//     };
// }

// // Implement LazyStatic for a struct
// pub struct ConfigManager;

// impl LazyStatic for ConfigManager {
//     fn init() {
//         println!("Configuration Loaded!");
//     }
// }

// fn main() {
//     // ConfigManager::init();
//     println!("Server Host: {}", CONFIG.get("server_host").unwrap());
// }







// use lazy_static::lazy_static;
// use std::sync::Mutex;

// // Define the LazyStatic trait
// pub trait LazyStatic {
//     fn init();
// }

// // Global counter stored in a Mutex
// lazy_static! {
//     static ref COUNTER: Mutex<i32> = Mutex::new(0);
// }

// // Implement LazyStatic for GlobalCounter
// pub struct GlobalCounter;

// impl LazyStatic for GlobalCounter {
//     fn init() {
//         let mut counter = COUNTER.lock().unwrap();     // COUNTER.lock() → returns a Result<MutexGuard<T>, PoisonError<T>>, meaning locking can fail if another thread panicked while holding the lock.
//         *counter = 10;                                 //.unwrap() → If locking succeeds, it returns a MutexGuard<i32>, allowing access to the inner integer (i32).
//         println!("Counter initialized to {}", *counter);
//     }
// }

// fn main() {
//     GlobalCounter::init();
    
//     // Modify the counter safely
//     {
//         let mut counter = COUNTER.lock().unwrap();
//         *counter += 5;
//     }

//     println!("Updated Counter: {}", *COUNTER.lock().unwrap());
// }








// //Lazy Static Resources: The init() method is typically called when the resource is accessed for the first time, triggering the initialization.
// use lazy_static::lazy_static;
// use std::sync::Mutex;

// // Define the LazyStatic trait
// pub trait LazyStatic {
//     fn init() -> String;           //The init() method allows you to specify the initialization logic for the resource or value.
// }

// // Define a global lazy static resource
// lazy_static! {
//     static ref CONFIG: Mutex<String> = Mutex::new(String::new());
// }

// // Implement the LazyStatic trait for CONFIG
// pub struct ConfigManager;

// impl LazyStatic for ConfigManager {
//     fn init() -> String {
//         let mut config = CONFIG.lock().unwrap();
//         *config = String::from("Configuration Loaded at Runtime!");
//         config.clone()  // Return a copy of the initialized config string
//     }
// }

// fn main() {
//     // Call the trait's init method
//     let config = ConfigManager::init();
//     println!("{}", config);
// }










// //RWLock
// use std::sync::RwLock;

// fn main() {
//     // Create an RwLock that wraps an integer
//     let counter = RwLock::new(0);

//     // Read the value
//     {
//         let read_data = counter.read().unwrap(); // Lock acquired for reading
//         println!("Read value: {}", *read_data);
//     } 

//     {
//         let mut write_data = counter.write().unwrap(); // Lock acquired for writing
//         *write_data += 1;
//         println!("Written value: {}", *write_data);
//     } 

//     {
//         let final_data = counter.read().unwrap(); // Lock acquired for reading
//         println!("Final value: {}", *final_data);
//     }
// }




// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     pub(crate) static ref CACHE: std::sync::RwLock<Vec<i32>> = std::sync::RwLock::new(vec![1, 2, 3]);
// }

// fn main() {
//     // Access the CACHE within the same crate
//     let cache = CACHE.read().unwrap();
//     println!("Cache: {:?}", *cache);
// }






// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     pub static ref CONFIG: String = String::from("Configuration Loaded");
// }

// fn main() {
//     println!("{}", *CONFIG); // Accessing the public static reference
// }







// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     static ref COUNTER: std::sync::Mutex<i32> = std::sync::Mutex::new(0);
// }

// fn main() {
//     let mut counter = COUNTER.lock().unwrap();
//     *counter += 1;
//     println!("Counter: {}", *counter);
// }

