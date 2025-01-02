console.log("y");
const readline = require('readline');
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.question('Enter x, y, z: ', (input) => {
  const [x, y, z] = input.split(' ').map(Number);
  console.log(x);
  
  console.log(x > y);
  
  console.log(z);
  
  console.log(x > z);
  
  console.log(y > z);
  
  rl.close();
});