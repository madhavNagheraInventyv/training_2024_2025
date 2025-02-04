const mongoose = require("mongoose");

const ReportSchema = new mongoose.Schema({
  timestamp: { type: Date, default: Date.now },
  reportData: { type: Object, required: true },
});

const Report = mongoose.model("Report", ReportSchema);

module.exports = Report;
