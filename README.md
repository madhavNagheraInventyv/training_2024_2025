# training_2024_2025

Aim: To establish a centralized, well-organized folder structure for managing and maintaining projects across multiple programming languages and technologies, ensuring scalability, accessibility, and consistency for future development, testing, and collaboration.

# Key Objectives  #
* Modularity:
Organize projects into separate subfolders by language, framework, or technology to simplify navigation.

* Scalability:
Enable easy addition of new projects, languages, or frameworks without compromising folder clarity.

* Reusability:
Include common assets, shared libraries, and documentation that can be reused across projects.

* Documentation:
Maintain clear documentation for each project and the main folder (e.g., README.md files) to facilitate onboarding and reference.

# file structure #
```
|-html_css
|    |-Task1
|    |-Task2
|
|-js
|  |-task1
|  |-task2
|   
|-flowcharts
|       |── areaofcircle.js
|       ├── Factorialofn.js
|       ├── maxamongX&Y&Zz.js
|       ├── maxamonfX&Y.js
|       ├── parameterOfRectangle.js
|       ├── print1to5.js
|       ├── series.js
|       ├── series2.js
|       ├── series3.js
|       ├── factorialseries.js
|       ├── factorialseries1.js
|       ├── sinx.js
|       ├── sumOf_n.js
|       └── README.md
|
|-Rust-Rangers
|        |-Madhav-rust
|        |        |-src
|        |           |-levels     
|        
|        
|- Circle_area_app
|        |──features/                           #the gherkin file for the project
|        ├── node_modules/                      # Project dependencies
|        |        |--circle.feature
|        ├── public/                            #Front-end
|        |        ├── index.html                # The main HTML file for the app
|        |        ├── script.js                 # The JavaScript file containing the app logic
|        |        └── style.css                 # The CSS file for styling the app
|        |                  
|        ├── test-results/                      # Folder for storing test results
|        |        |──.last.run.json
|        ├── tests/                             # Folder containing feature files and step definitions for Cucumber
|        |        |──step_definitions/
|        |        |        |--circle.steps.js
|        |        |──playwright.test.js
|        |        |──README.md
|        ├── cucumber.js                        # Cucumber configuration file
|        ├── package-lock.json                  # Lock file for npm dependencies
|        ├── package.json                       # Project metadata and dependencies
|        ├── playwright.config.js               # Playwright configuration file
|        ├── README.md                          # This file
|        └── server.js                          # Node.js server to run the app
|- Test_middleware-api-rust/
|         │   
|         ├── middleware_axum/                   #Contains the Axum API with request counting middleware.       
|         │   │   ├── main.rs
|         │   ├── Cargo.toml
|         ├── test_api/                          #A simple API endpoint (`/api/test`) that logs a message when accessed.
|         │   ├── src/
|         │   │   ├── main.rs
|         │   ├── Cargo.toml
|         ├── test_api2/            #API that handles a `user/{id}` route, returning a JSON response based on the `id`.
|         │   ├── src/
|         │   │   ├── main.rs
|         │   ├── Cargo.toml
|         ├── README.md
   





      
```    


# Contributing #
Feel free to contribute by adding new front-end projects, enhancing existing features, or fixing bugs. Make sure your work adheres to the "No JavaScript" policy for this folder.
