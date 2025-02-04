# Cucumber Playwright Automation with MinIO

This project implements automated testing using **Cucumber** and **Playwright**, integrating **MinIO** to capture and store test execution videos.

## 📌 Features

- ✅ End-to-end testing with **Cucumber** and **Playwright**.
- 🎥 Video recording of test executions.
- 🏗️ MinIO integration for storing test videos.
- 📊 Test reporting and result storage.

## 🛠️ Prerequisites

Ensure you have the following installed:

- [Node.js](https://nodejs.org/) (LTS recommended)
- [MinIO](https://min.io/) (Self-hosted or cloud setup)
- [Docker](https://www.docker.com/) (Optional for MinIO setup)
- [Playwright](https://playwright.dev/)
- [Cucumber.js](https://github.com/cucumber/cucumber-js)

## 🚀 Setup Instructions

### Install the minio exe

[exe file for minio](https://dl.min.io/server/minio/release/windows-amd64/minio.exe)




---
## Test Features
```bash
Feature: Search Rust Books with Keywords

  Scenario: Search for Rust books by keyword "Concurrency"
    Given I am on the Rust Book search page
    When I search for "Understanding Ownership"
    Then I should see Rust books with "Concurrency" in the title or description
```


## File Structure
```
cucumber-playwright-minio/
|-models/
|      |-report.js
|-reports/
|      |-cucumber-report.html
|      |-cucumber-report.json
|
|-test/
|      |-features/
|      |       |-site.feature
|      |-step_definitions
|              |-minio.js
|              |-site.step.js
|
|-couchbase.js
|-cucumber.js
|-db.js
|-package-lock.json
|-package.json
|-README.md
|-server.js
|-storeReport.js
```