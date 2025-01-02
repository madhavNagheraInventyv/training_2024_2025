const readline = require("readline");

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});

function factorial(n) {
    let result = 1;
    for (let i = 1; i <= n; i++) {
        result *= i;
    }
    return result;
}

rl.question("Enter a number: ", (nInput) => {
    const n = parseInt(nInput);

    console.log(`Factorial of ${n} is ${factorial(n)}`);

    rl.close();
});
