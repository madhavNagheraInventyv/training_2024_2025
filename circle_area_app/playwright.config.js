import { defineConfig } from '@playwright/test';
import path from 'path';

export default defineConfig({
  use: {
    baseURL: 'http://localhost:3000',
    headless: false,
  },
  reporter: [
    ['html', { outputFolder: path.join('circle_area_app', 'reports', 'playwright-report'), open: 'never' }],
    ['json', { outputFile: path.join('circle_area_app', 'reports', 'playwright-report', 'report.json') }],
  ],
});
