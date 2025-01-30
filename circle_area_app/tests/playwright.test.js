const { test, expect } = require('@playwright/test');

test('Calculate area of circle with Playwright', async ({ page }) => {
    await page.goto('http://localhost:3000');
    await page.fill('#radius', '5');
    await page.click('button');
    await expect(page.locator('#result')).toHaveText(/Area: 78.54/);
});