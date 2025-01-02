const readline = require("readline");

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});

function calculateSeriesSum(n, x) {
    let count = 1;
    let i = 1;
    let mul = 1;
    let sum = 0.0;

    while (count <= n) {
        let pow = 1;
        let fact = 1;
        let k = 1;

        while (k <= i) {
            pow *= x;
            fact *= k;
            k++;
        }

        sum += mul * (pow / fact);

        i += 2;
        count++;
        mul *= -1; 
    }

    return sum.toFixed(2);
}


rl.question("Enter the value of n: ", (nInput) => {
    const n = parseInt(nInput);

    rl.question("Enter the value of x: ", (xInput) => {
        const x = parseInt(xInput);

        const result = calculateSeriesSum(n, x);
        console.log(`The sum of the series is: ${result}`);

        rl.close();
    });
});
