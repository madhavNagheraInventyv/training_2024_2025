const { Given, When, Then, Before, After } = require('@cucumber/cucumber');
const { chromium, expect } = require('@playwright/test');

let browser;
let page;

Before(async () => {
  browser = await chromium.launch({ headless: true });
  page = await browser.newPage();
});

After(async () => {
  await browser.close();
});

Given('I am on the Rust documentation search page', async function () {
  await page.goto('https://doc.rust-lang.org/book/', { timeout: 5000 });
  await page.waitForLoadState('networkidle');
});

When('I enter the following book titles in the {string} search field and click the {string} button:', async function (searchField, buttonText, dataTable) {
  const bookTitles = dataTable.rawTable.map(row => row[0]);

  // Loop through the titles and enter them into the search field
  for (const title of bookTitles) {
    await page.fill(searchField, title);
    await page.click(buttonText);
  }
});

Then('I should see the books with the following titles in the results:', async function (dataTable) {
  const expectedTitles = dataTable.rawTable.map(row => row[0]);

  // Wait for search results to load
  await page.waitForSelector('.search-result-title', { timeout: 5000 });

  // Get all result titles
  const resultTitles = await page.locator('.search-result-title').allTextContents();

  // Validate each result title contains the expected text
  for (const title of expectedTitles) {
    expect(resultTitles).toContain(title);
  }
});
