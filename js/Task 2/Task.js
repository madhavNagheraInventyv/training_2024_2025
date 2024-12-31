let arr1 = [1, 2, 3, 4, 5];

(function IIFE() {
    let element = arr1.shift();
    function2(element, ...arr1);
})();

function function2(element, ...restArr1) {
    let arr2 = [6, 7, 8, 9];
    arr2.unshift(element);

    for (let i = 0; i < restArr1.length; i++) {
        arr2[arr2.length] = restArr1[i];
    }
    let sum = 0;
    for (let i = 0; i < arr2.length; i++) {
        sum += arr2[i];
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