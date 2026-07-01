# Types of Tests

## Unit Tests

Simple automated test to prove the behavior of a method.

- Requires mocks for external services
- Are independent
- Verifies whether the method works as expected

Example:

Imagine a function that should return `"Hello, {Name}!"` and I want to test whether I get that return when I call it, so I can call it passing a fake name
and see if the result obtained matches what I expected. Example:

~~~
helloName('Carol') // return "Hello, Carol!"
~~~

## Integration Tests

Test to validate whether services are working together.

- Does not use mocks
- Uses real return values from services to validate

Example:

Now imagine I have a function called `printName` that, when a user types their name in a popup, calls the `helloName` function and prints its return to the screen. I'll test whether `printName` picks up the right name to pass to `helloName`, whether the return from `helloName` is what's expected, and finally whether I'm successfully printing it on the screen. I tested the component working as a whole with real values and returns.

## e2e Tests

Test to analyze the external part of the application — its usability.

- Validates the application's operational flow

## Difference between unit, integration, and e2e

<p align="center">
  <img width="300px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/unit.gif" />
  <img width="300px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/integrado.gif" />
  <img width="300px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/e2e.gif" />
</p>

Unit tests mock services and test only the function as a unit. Integration tests check whether the method or component is working well alongside its dependencies. e2e tests cover usability and navigation itself.

Example:

Now I want to test whether, when I open my website, a popup appears asking for my name, and whether when I type my name and submit, I receive a greeting with my name on screen. That is an e2e test.

## Differences between BDD and TDD

<p align="center">
  <img width="600px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/tdd-vs-bdd.png" />
</p>

TDD (Test Driven Development) is a software development methodology that aims to write tests first, and only then write functions that pass those tests. BDD (Behavior Driven Development) is an agile development technique that presents the developer with the expected behavior of a given feature and lets them choose the best way to implement it — and just like in TDD, tests are written first and code comes after. The difference between the two is that in TDD the function is defined, while in BDD the behavior is defined, but the choice of how to execute it is the developer's responsibility.

### TDD - red-green-refactor

- You create a test for a function
- It fails because you're testing something that doesn't work yet
- Then you write a "dumb" implementation just to make the test pass
- Now it's time to refactor: improve the code and turn it into something truly good

Advantages: a much cleaner code design, free of redundancies, and much more scalable and flexible code.

### BDD give-when-then

- Initial state, actions, expected outcomes
- Involves everyone responsible for the product
- Must have readable requirements
- Requirements must be converted into executable tests
- Uses mocks for tests of features that haven't been written yet

Dan, the creator of BDD, says it is an evolution of TDD. For him, development should go far beyond the technical team and bring together everyone involved in the product.

## Functional or non-functional tests

### Functional tests or black-box tests

This concept covers all tests that test behavior (inputs and outputs):

- Unit tests
- Integration tests
- Acceptance tests
- System tests

### Non-functional tests

The concept of non-functional tests refers to all tests that are not related to functional behavior. Some examples of non-functional tests:

- Performance
- Load/stress testing
- Whether the UI is intuitive and easy to learn

## :tada: AND FINALLY :tada:

The user is not the tester

<p align="center">
  <img src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/testado-no-ar.gif" />
</p>
