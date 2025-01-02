const readline = require("readline");

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});

const PI = 3.14159;

console.log("Calculate the area of a circle:");

rl.question("Enter the radius of the circle: ", (radiusInput) => {
    const radius = parseFloat(radiusInput);

    const area = PI * radius * radius;

    console.log(`The area of the circle is: ${area.toFixed(2)}`);
    rl.close();
});
