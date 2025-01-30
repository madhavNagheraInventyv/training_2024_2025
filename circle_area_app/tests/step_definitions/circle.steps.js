const { Given, When, Then } = require('@cucumber/cucumber');
const puppeteer = require('puppeteer');
const { expect } = require('chai');

let browser, page;

Given('I open the calculator page', async () => {
    browser = await puppeteer.launch();
    page = await browser.newPage();
    await page.goto('http://localhost:3000');
});

When('I enter radius {string} and press calculate', async (radius) => {
    await page.type('#radius', radius);
    await page.click('button');
    await page.waitForTimeout(1000);
});

Then('I should see the area displayed as {string}', async (expectedArea) => {
    const resultText = await page.$eval('#result', el => el.innerText);
    expect(resultText).to.include(expectedArea);
    await browser.close();
});

Then('I should see an error message', async () => {
    const resultText = await page.$eval('#result', el => el.innerText);
    expect(resultText).to.include('Invalid radius');
    await browser.close();
});