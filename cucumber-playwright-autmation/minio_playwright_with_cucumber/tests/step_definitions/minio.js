const { Given, When, Then, Before, After, setDefaultTimeout } = require("@cucumber/cucumber");
const { expect } = require("@playwright/test");
const { chromium } = require("@playwright/test");
const fs = require("fs");
const path = require("path");
const Minio = require("minio");

setDefaultTimeout(30 * 1000);

const minioClient = new Minio.Client({
  endPoint: "localhost",
  port: 9000,
  useSSL: false,
  accessKey: "minioadmin",
  secretKey: "minioadmin",
});

const BUCKET_NAME = "test-results";

Before(async function () {
  // Define video storage folder
  this.browser = await chromium.launch({ headless: false }); // Launch browser
  this.context = await this.browser.newContext({
    recordVideo: { dir: "./videos", size: { width: 1280, height: 720 } }, // Enable video recording
  });

  this.page = await this.context.newPage(); // Create a new page
});

After(async function () {
  try {
    const videoPath = await this.page.video().path();
    if (videoPath) {
      const videoFileName = path.basename(videoPath);

  
      const bucketExists = await minioClient.bucketExists(BUCKET_NAME);
      if (!bucketExists) {
        await minioClient.makeBucket(BUCKET_NAME);
      }

      
      await minioClient.fPutObject(BUCKET_NAME, videoFileName, videoPath);
      console.log(` Video uploaded to MinIO: ${videoFileName}`);
    } else {
      console.error(" No video file recorded.");
    }
  } catch (error) {
    console.error(" Error during After hook:", error);
  } finally {
    if (this.page) await this.page.close();
    if (this.browser) await this.browser.close();
  }
});

// Rust Book Search Scenario
Given("I am on the Rust Book search page", async function () {
  await this.page.goto("https://doc.rust-lang.org/book/");
});

When("I search for {string}", async function (searchQuery) {
  // Check if the page is still open before interacting with it
  if (this.page && !this.page.isClosed()) {
    const searchInput = await this.page.locator('input[name="search"]');
    
    // Wait for the input to be visible before filling it
    await searchInput.waitFor({ state: 'visible', timeout: 5000 });
    await searchInput.fill(searchQuery);
    await searchInput.press("Enter"); // Press Enter to perform search
  } else {
    console.error("❌ The page was closed before the operation could be completed.");
  }
});


Then("I should see Rust books with {string} in the title or description", async function (expectedKeyword) {
  // Wait for the search results to appear
  await page.waitForSelector('.search-result-title', { timeout: 5000 });

  // Get the list of search result titles and descriptions
  const resultTitlesAndDescriptions = await page.locator('.search-result-title').allTextContents();

  // Ensure each result contains the expected keyword in the title or description
  for (const content of resultTitlesAndDescriptions) {
    expect(content.toLowerCase()).toContain(expectedKeyword.toLowerCase());
  }
});
