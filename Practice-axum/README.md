# Axum Practice Project

This project is designed to practice various features of Axum, a web framework in Rust. It is structured into three main folders: **basic**, **lazy-static**, and **usertest**, each focusing on different aspects of Axum development.

## Folder Structure

- **basic**: Contains examples for different Axum methods to practice.
- **lazy-static**: Contains code using `lazy_static` for `Mutex` and `RwLock` to handle concurrency.
- **usertest**: Focuses on user registration, login, and handling car fields as structs in a TiDB database using SQLx.

## Features

### 1. Basic Axum Methods

In the `basic` folder, you will find various methods to practice setting up Axum routes and handling HTTP requests such as:

- GET, POST, DELETE requests
- Path and query parameters
- Request bodies and JSON handling
- Axum middleware setup

### 2. Lazy Static with Mutex and RwLock

The `lazy-static` folder demonstrates how to use `lazy_static` to initialize a static value in your application. This folder contains examples for:

- Using `Mutex` to handle mutable state safely across threads
- Using `RwLock` for read-write locking, allowing multiple readers but exclusive writing

### 3. User Registration, Login, and Car Fields in TiDB using axum (SQLx)
## User Authentication and Token Handling

### User Registration

When a new user registers, the following steps are performed:

1. **User Data**: The user provides their registration details, such as username and password.
2. **Password Hashing**: The password is hashed for security purposes using a secure hashing algorithm (e.g., `bcrypt`, `argon2`, etc.) before being stored in the database.
3. **Token**: At the time of registration, the token is **set to null** in the database since the user has not yet logged in or been authenticated.
4. **Database**: The user details (username, hashed password, and null token) are saved in the `user` table of the database.

```sql
-- Example query for registering a user
INSERT INTO user (username, password_hash, token) VALUES (?, ?, NULL);
```
- User registration and login is saved in users table
- Car fields data is stored in TiDB subsystem using SQLx in cars table
g

## Middleware Overview

1. **Token Middleware**: Extracts the token from the request headers and makes it available for authentication.
2. **Authentication Middleware**: Authenticates users by verifying the provided token using the database, ensuring that requests are made by authorized users.
3. **Logging Middleware**: Logs details about each request and response for monitoring purposes.

## Prerequisites

Before running this project, make sure you have the following dependencies installed:

- Rust (latest stable version)
- Axum
- SQLx
- TiDB (or a compatible database)
- `tokio` runtime

## Running the Project

To run the project, navigate to the folder of the desired section (e.g., `basic`, `lazy-static`, `usertest`) and use `cargo run` to start the server. Make sure you have the necessary environment setup for TiDB and the database connection string configured properly.

```bash
cargo run
```


## File Structure

```bash
axum-practice-project/
│
├── basic/
│   ├── src/
│   │   └── handlers/        # different-different axum methods to practice
│   │    └── lib.rs          # sets all the endpoint for the file
│   │    └── main.rs         # Main entry point with various Axum methods (GET, POST, PUT, DELETE)
│   └── Cargo.toml          # Dependencies for Axum and related packages
│
├── lazy-static/
│   ├── src/
│   │   └── main.rs         # Code using lazy_static alogn with Mutex and RwLock for concurrency
│   └── Cargo.toml          # Dependencies for lazy_static and concurrency management
│
├── usertest/
│   ├── src/
│   │   ├── handlers/       # User registration, login, and car fields handling with TiDB and SQLx 
|   │   └── middleware/     # Middleware for token passing, authentication, and logging
│   │   └── routes          # contains code for ssrf and csrf but integration is not done yet
│   │   └── lib.rs          # sets all the endpoint for the file
│   │   └── models.rs       # Define user and car structs, and SQLx database interactions
│   └── Cargo.toml          # Dependencies for SQLx, TiDB, and middleware setup
│
└── README.md               # Project overview and documentation (this file)





```