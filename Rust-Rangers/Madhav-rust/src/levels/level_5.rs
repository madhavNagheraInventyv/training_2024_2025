use std::io;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
pub async fn level5() {
    println!("welcome to The Fields of Structs and Enums");
    sleep(Duration::from_millis(2000)).await;
    println!("loading....!");
    sleep(Duration::from_millis(2000)).await;


    #[derive(Debug, PartialEq)]
    enum CarCategory {
        Hatchback,
        Sedan,
        SUV,
        Sports,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Car {
        name: String,
        top_speed: u32,
        seating_capacity: u8,
        price: String,
        category: CarCategory,
    }

    let cars = vec![
        Car {
            name: String::from("Grand i10"),
            top_speed: 165,
            seating_capacity: 5,
            price: String::from("8L"),
            category: CarCategory::Hatchback,
        },
        Car {
            name: String::from("Honda Civic"),
            top_speed: 220,
            seating_capacity: 5,
            price: String::from("22L"),
            category: CarCategory::Sedan,
        },
        Car {
            name: String::from("Toyota Fortuner"),
            top_speed: 200,
            seating_capacity: 7,
            price: String::from("60L"),
            category: CarCategory::SUV,
        },
        Car {
            name: String::from("RR Sports"),
            top_speed: 220,
            seating_capacity: 5,
            price: String::from("1.3cr"),
            category: CarCategory::Sports,
        },
    ];

    loop {
        println!("Enter the category number of the car you want:");
        println!("1 => Hatchback");
        println!("2 => Sedan");
        println!("3 => SUV");
        println!("4 => Sports");
        println!("5 => Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "1" => print_cars(&cars, CarCategory::Hatchback),
            "2" => print_cars(&cars, CarCategory::Sedan),
            "3" => print_cars(&cars, CarCategory::SUV),
            "4" => print_cars(&cars, CarCategory::Sports),
            "5" => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid input. Please enter a valid category number."),
        }
    }


    fn print_cars(cars: &[Car], category: CarCategory) {
        println!("Cars in the {:?} category:", category);
        for car in cars {
            if car.category == category {
                println!("{:#?}", car);
            }
        }
    }
    
}
