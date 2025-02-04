const express = require("express");
const couchbase = require("couchbase");
const path = require("path");
const fs = require("fs");
const app = express();
require('dotenv').config(); 
app.use(express.json()); // To parse JSON requests
const clusterConnStr = process.env.COUCHBASE_CLUSTER_CONN_STR;
const username = process.env.COUCHBASE_USERNAME;
const password = process.env.COUCHBASE_PASSWORD;
const bucketName = process.env.COUCHBASE_BUCKET_NAME;
let cluster, bucket, collection;

// Function to connect to Couchbase
async function connectToCouchBase() {
    try {
        cluster = await couchbase.connect(clusterConnStr, {
            username: username,
            password: password,
            kvTimeout: 30000
        });

        bucket = cluster.bucket(bucketName);
        collection = bucket.defaultCollection();

        console.log("Connected to Couchbase");
    } catch (err) {
        console.error("Connection failed:", err);
        process.exit(1); // Exit process if connection fails
    }
}

// Call the connection function and start the server only after success
connectToCouchBase().then(() => {
    const PORT = 3000;
    app.listen(PORT, () => {
        console.log("Server running on http://localhost:${PORT}");
    });
});



app.post("/store", async (req, res) => {
    if (!collection) {
        return res.status(500).json({ error: "Database connection failed" });
    }

    try {
        const filePath = path.join(__dirname, "reports", "cucumber-report.json");

        if (!fs.existsSync(filePath)) {
            return res.status(404).json({ error: "cucumber-report.json not found!" });
        }

        const reportData = fs.readFileSync(filePath, "utf-8");

      
        const parsedData = JSON.parse(reportData);

       
        const newId = `report-${Date.now()}`; 

        const rustBookData = {
            id: newId,  
            report: parsedData, 
            createdAt: new Date(),
        };

    
        await collection.upsert(rustBookData.id, rustBookData);

        // Log the success message
        console.log('Cucumber Report stored successfully with ID:', newId);

        res.status(201).json({ message: "Cucumber Report stored successfully", data: rustBookData });
    } catch (err) {
        console.error("Error storing Rust book data:", err);
        res.status(500).json({ error: "Internal Server Error", details: err.message });
    }
});

app.get("/get-rustbook-data/:id", async (req, res) => {
    if (!collection) {
        return res.status(500).json({ error: "Database connection failed" });
    }

    try {
        const { id } = req.params;
        const result = await collection.get(id); // Fetch data by id

        res.status(200).json({ message: "Data retrieved successfully", data: result.value });
    } catch (err) {
        console.error("Error retrieving Rust book data:", err);
        res.status(500).json({ error: "Internal Server Error" });
    }
});
