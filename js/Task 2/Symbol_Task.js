let arr1 = [Symbol(1), Symbol(2), Symbol(3), Symbol(4), Symbol(5)];

(function IIFE() {
    let element = arr1.shift();
    function2(element, ...arr1);
})();

function function2(element, ...restArr1) {
    let arr2 = [Symbol(6), Symbol(7), Symbol(8), Symbol(9)];
    arr2.unshift(element);

    // Manually append restArr1 to arr2 without using push
    for (let i = 0; i < restArr1.length; i++) {
        arr2[arr2.length] = restArr1[i];
    }

    // Convert symbols to their respective values for summation
    let sum = 0;
    for (let i = 0; i < arr2.length; i++) {
        sum += Number(arr2[i].description);
    }

    let promise = new Promise((resolve, reject) => {
        if (sum > 30) {
            resolve("Sum is greater than 30");
        } else {
            reject("Sum is less than or equal to 30");
        }
    });

    promise
        .then(message => console.log(message))
        .catch(error => console.log(error));
}