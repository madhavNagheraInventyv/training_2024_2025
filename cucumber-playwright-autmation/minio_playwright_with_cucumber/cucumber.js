module.exports = {
  default: {
    require: ["tests/step_definitions/*.js"],
    // require: ["tests/step_definitions/minio.js"],        // to run with minio uncomment this
    format: ["progress", "json:reports/cucumber-report.json", "html:reports/cucumber-report.html"], // JSON report output
    paths: ["tests/features/*.feature"],
    worldParameters: {},
  },
};
