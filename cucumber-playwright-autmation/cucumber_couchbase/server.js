const express = require("express");
const fs = require("fs");
const path = require("path");
const mongoose = require("mongoose");
const cors = require("cors");
const connectDB = require("./db");
const Report = require("./models/report");
const { exec } = require("child_process");
const app = express();
const PORT = 3000;
const jsonReportPath = "reports/cucumber-report.json";

app.use(cors());
app.use(express.json());

connectDB().then(async () => {
    console.log("🚀 Storing report in MongoDB...");

    if (!fs.existsSync(jsonReportPath)) {
      console.error("❌ JSON report not found!");
      return;
    }
  
    const reportData = JSON.parse(fs.readFileSync(jsonReportPath, "utf-8"));
  
    try {
      const newReport = new Report({ reportData });
      await newReport.save();
      console.log("✅ Report saved successfully in MongoDB!");
    } catch (error) {
      console.error("❌ Error saving report:", error);
    }
  });   

// app.get("/", (req, res) => {
//     res.send(`<a href="http://localhost:5000/generate">generate Report</a>`);
// });

// app.get("/generate", (req, res) => {
//     exec("npm run test", (error, stdout, stderr) => {
//         if (error) {
//             console.error(`exec error: ${error}`);
//             return;
//         }
//         console.log(`stdout: ${stdout}`);
//         console.error(`stderr: ${stderr}`);
//         res.redirect('/reports');
//     });   
// });
app.get("/reports", async (req, res) => {
  try {
    const reports = await Report.find().sort({ timestamp: -1 });
    res.json(reports);
  } catch (error) {
    res.status(500).json({ error: "Internal Server Error" });
  }
});

app.get("/latest-report", (req, res) => {
  const htmlReportPath = path.join(__dirname, "reports", "cucumber-report.html");

  if (fs.existsSync(htmlReportPath)) {
    res.sendFile(htmlReportPath);
  } else {
    res.status(404).json({ error: "Report not found!" });
  }
});

app.listen(PORT, () => {
  console.log(`🚀 Server running on http://localhost:${PORT}`);
});
