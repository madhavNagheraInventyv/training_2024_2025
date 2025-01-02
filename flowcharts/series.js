const readline = require("readline");

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});

rl.question("Enter a number: ", (nInput) => {
    const n = parseInt(nInput);
    let mul = 1;
    let i = 1;

    while (i <= n) {
        process.stdout.write(`${i * mul}, `);
        mul = mul * -1;
        i = i + 1;
    }

    rl.close();
});
