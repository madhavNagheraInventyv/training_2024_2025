# Cucumber Playwright Automation with Couchbase

This project implements automated testing using **Cucumber** and **Playwright**, integrating **Couchbase** to store the entire generated JSON test report in a local Couchbase bucket.

## 📌 Features

- ✅ End-to-end testing with **Cucumber** and **Playwright**.
- 📜 JSON test report generation.
- 🏗️ Couchbase integration for storing test reports.
- 📊 Test reporting and result storage.

## 🛠️ Prerequisites

Ensure you have the following installed:

- [Node.js](https://nodejs.org/) (LTS recommended)
- [Couchbase Server](https://www.couchbase.com/)
- [Docker](https://www.docker.com/) (Optional for Couchbase setup)
- [Playwright](https://playwright.dev/)
- [Cucumber.js](https://github.com/cucumber/cucumber-js)

## 🚀 Setup Instructions

### Install Couchbase Server

1. Download Couchbase Server from:
   - [Couchbase Download Page](https://www.couchbase.com/downloads)
   - Select **Community Edition** (or Enterprise if you have a license).

2. Start Couchbase Server and create a **local bucket** named `test-reports`.

3. Modify the `.env` file with Couchbase credentials:

```env
COUCHBASE_HOST=localhost
COUCHBASE_BUCKET=test-reports
COUCHBASE_USER=admin
COUCHBASE_PASSWORD=password
```

## Test Features
```gherkin
Feature: Search Rust Books with Keywords

  Scenario: Search for Rust books by keyword "Concurrency"
    Given I am on the Rust Book search page
    When I search for "Understanding Ownership"
    Then I should see Rust books with "Concurrency" in the title or description
```
## File Structure
```bash
cucumber-playwright-couchbase/
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