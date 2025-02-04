module.exports = {
  default: {
    require: ["tests/step_definitions/**/*.js"],
    format: ["progress", "json:reports/cucumber-report.json", "html:reports/cucumber-report.html"], // JSON report output
    paths: ["tests/features/*.feature"],
    worldParameters: {},
  },
};
