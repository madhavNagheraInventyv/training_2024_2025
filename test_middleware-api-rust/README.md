# Test_middleware-api-rust

This project demonstrates an **Axum** API with middleware functionality, API routing, and handling user data. It includes three separate Axum applications, each focused on a specific functionality: middleware usage, a simple API test, and user data fetching.

## Project Structure
```
Test_middleware-api-rust/
      │   ├── src/
      ├── middleware_axum/                     
      │   │   ├── main.rs
      │   ├── Cargo.toml
      ├── test_api/
      │   ├── src/
      │   │   ├── main.rs
      │   ├── Cargo.toml
      ├── test_api2/
      │   ├── src/
      │   │   ├── main.rs
      │   ├── Cargo.toml
      ├── README.md


```

- **middleware_axum/**: Contains the Axum API with request counting middleware.
- **test_api/**: A simple API endpoint (`/api/test`) that logs a message when accessed.
- **test_api2/**: API that handles a `user/{id}` route, returning a JSON response with user data based on the `id`.

## Prerequisites

- Rust 1.56 or higher
- Cargo (Rust package manager)
- Dependencies: Axum, Hyper, Tokio, Serde

## Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/your-username/Test_middleware-api-rust.git
   cd Test_middleware-api-rust
