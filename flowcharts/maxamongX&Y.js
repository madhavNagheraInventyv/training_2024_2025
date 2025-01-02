const readline = require("readline");

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});

rl.question("Enter two numbers separated by space: ", (input) => {
    const [num1, num2] = input.split(" ").map(Number);

    const max = (num1 > num2) ? num1 : num2;

    console.log(`Maximum of the two numbers is: ${max}`);

    rl.close();
});
