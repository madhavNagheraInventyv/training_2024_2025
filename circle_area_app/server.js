const express = require('express');
const path = require('path');
const app = express();
const port = 3000;

app.use(express.static('public'));

app.get('/area', (req, res) => {
    const radius = parseFloat(req.query.radius);
    if (isNaN(radius) || radius <= 0) {
        return res.status(400).json({ error: 'Invalid radius' });
    }
    const area = Math.PI * radius * radius;
    res.json({ radius, area });
});

app.listen(port, () => {
    console.log(`Server running at http://localhost:${port}`);
});