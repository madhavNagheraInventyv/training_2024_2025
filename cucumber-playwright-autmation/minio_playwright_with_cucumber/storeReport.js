const fs = require("fs");
const mongoose = require("mongoose");
const connectDB = require("./db");
const Report = require("./models/report");

const jsonReportPath = "reports/cucumber-report.json";

// Function to store report
const storeReport = async () => {
  await connectDB(); // Ensure DB connection

  console.log("🚀 Storing new test report in MongoDB...");

  if (!fs.existsSync(jsonReportPath)) {
    console.error("❌ JSON report not found!");
    process.exit(1);
  }

  const reportData = JSON.parse(fs.readFileSync(jsonReportPath, "utf-8"));

  try {
    const newReport = new Report({ reportData, timestamp: new Date() });
    await newReport.save();
    console.log("✅ Report saved successfully in MongoDB!");
  } catch (error) {
    console.error("❌ Error saving report:", error);
  } finally {
    mongoose.connection.close(); // Close connection after storing
  }
};

storeReport();
