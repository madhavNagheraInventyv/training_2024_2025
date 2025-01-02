const readline = require("readline");

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});


rl.question("Enter a number: ", (input) => {
    const n = parseInt(input); 

    let id = 1; 
    let fact = 1; 

    while (id <= n) {
        console.log(`${id}:${fact}`); 
        id += 2; 
        fact *= id * (id - 1);
    }

    rl.close(); 
});
