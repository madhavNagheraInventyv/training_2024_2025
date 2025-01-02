const readline = require("readline");

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});

console.log("Calculate the perimeter of a rectangle:");

rl.question("Enter the length and width of the rectangle separated by space: ", (input) => {
    const [length, width] = input.split(" ").map(parseFloat);

    const perimeter = 2 * (length + width);

    console.log(`Perimeter of the rectangle is: ${perimeter.toFixed(2)}`);
    rl.close();
});
