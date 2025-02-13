# Circle Area Calculation App with Cucumber Playwright Testing

This project calculates the area of a circle and includes testing using Cucumber with Playwright, running on a Node.js server.

## Project Overview

This application provides functionality to calculate the area of a circle given a radius. The app is built using Node.js and has a test suite written with Cucumber and Playwright to ensure the application works correctly. The app also has a simple server that serves the functionality.

## Setup Instructions

### Prerequisites

Make sure you have the following installed:

- [Node.js](https://nodejs.org/) (version 14 or higher)
- [npm](https://www.npmjs.com/) (Node.js package manager)

### Installation

1. Clone the repository to your local machine:

    ```bash
    git clone <repository-url>
    cd circle_area_app
    ```

2. Install the required dependencies:

    ```bash
    npm install
    ```

3. The project uses Playwright for browser automation and Cucumber for behavior-driven development (BDD) testing. To install Playwright and the necessary testing libraries, run:

    ```bash
    npm install --save playwright cucumber
    ```

### Running the Server

To start the Node.js server, run the following command:

```bash
node server.js
npx playwright install
```
## File Structure

The project directory has the following structure:

```bash
circle_area_app/
    |──features/                           #the gherkin file for the project
    |        |--circle.feature
    ├── node_modules/                      # Project dependencies
    ├── public/                            #Front-end
    |        ├── index.html                # The main HTML file for the app
    |        ├── script.js                 # The JavaScript file containing the app logic
    |        └── style.css                 # The CSS file for styling the app
    |                  
    ├── test-results/                      # Folder for storing test results
    |        |──.last.run.json
    ├── tests/                             # Folder containing feature files and step definitions for Cucumber
    |        |──step_definitions/
    |        |        |--circle.steps.js
    |        |──playwright.test.js
    |        |──README.md
    ├── cucumber.js                        # Cucumber configuration file
    ├── package-lock.json                  # Lock file for npm dependencies
    ├── package.json                       # Project metadata and dependencies
    ├── playwright.config.js               # Playwright configuration file
    ├── README.md                          # This file
    └── server.js                          # Node.js server to run the app


```