# JavaScript
This section covers the fundamentals and advanced concepts of JavaScript, focusing on array manipulation, symbols, and promises. The tasks are designed to enhance your understanding and proficiency in using JavaScript for various operations and handling asynchronous processes.

# Key Objectives
* Understand and manipulate arrays in JavaScript.
Learn the use of symbols as unique identifiers.
Master the handling of promises for asynchronous operations.
* Design and implement a login/register page using HTML, CSS, and JavaScript.
   * Integrate form validation for user input.
   * Handle user authentication using JavaScript.
   * Implement storage and retrieval of user data using localStorage or sessionStorage.

# Task Synoposis

## Task 1
### Aim
To create a login register page using js and localstorage and a session time of 42secs.
#### Functional Requirements

* User Registration

* Users can register by providing a username and password.

* The credentials are stored in the browser's LocalStorage.

* User Login

* Registered users can log in by providing their credentials.

* Successful login initializes a session.

* Session Management

* A user session lasts for 42 seconds after login.

* If the session expires, the user is logged out automatically.

#### Error Handling

* Proper error messages for invalid credentials, duplicate registrations, and expired sessions.

* Data Persistence

* User credentials are securely stored in LocalStorage.

* No sensitive data (like plaintext passwords) is stored directly; hashed passwords should be used.

* Non-Functional Requirements

#### Responsiveness

* The interface should be accessible on various devices and screen sizes.

#### Performance

* Page load and operations (registration, login, session handling) should execute efficiently.

#### Security

* Basic security measures, such as password hashing and input validation, should be implemented.

* Ease of Use

* The user interface should be simple and intuitive.

#### Constraints

* The application must use JavaScript (no server-side processing).

* Data persistence should rely solely on LocalStorage.

* The session timeout should be precisely 42 seconds.

* External libraries or frameworks may be used but should not complicate the solution unnecessarily.



## Task 2

### Aim
In this task, we worked with JavaScript to manipulate arrays and symbols, and to handle promises.
Requirements




### Requirements
- Create and manipulate arrays using JavaScript.
- Use symbols as unique identifiers.
- Handle promises to manage asynchronous operations.

### Constraints

- Avoid use of `Reduce` and other in-built functions except `shift` and `unshift`.
- Use a promise if the sum of all elements is greater than 30; otherwise, do not use a promise.

### File structure ###
```
|-JavaScript
    |-task1
    |   |-test.html
    |   |-test.css
    |   |-dashboard.html
    |   |-images
    |       |- download.png
    |-Task 2
        |--README.md
        |--Symbol_Task.js
        |--Task.js
```