# Test Suites

### Suites and Describes

<p align="center">
  <img width="400px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/suite.gif" />
</p>

Imagine you have the file `say-hello.js` and want to test it. So you create `say-hello.spec.js`. In it you do the imports or requires of everything you'll need to
test your functions.

The next step is to call the `describe` method. It creates a group of tests — this group is called a `suite`. Describes receive a string with the
name of the test suite and a function that defines the inner suites and specs.

Example:

~~~
describe('sayHello', () => {
 // suites and specs go here
});
~~~

### Specs and its

<p align="center">
  <img width="400px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/it.gif" />
</p>

Now inside the describe we call the `it` method, which is what actually gets executed. It receives a string describing what is expected when something is executed (`should/when`) — this description is called a `spec` — and a function that contains the test code.

Example:

~~~
it('Expects to say hello when it receives a name', () => {
 // test code goes here
};
~~~

### Expectations

<p align="center">
  <img width="400px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/expectations.gif" />
</p>

The `expect` is what you expect to happen when you run that test. For example: when the `sayHello` method receives a name, you expect it to return the received name.

~~~
expect(sayHello('Carol')).to.be.a('Carol');
~~~


### Matchers

<p align="center">
  <img width="300px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/matcher.gif" />
</p>

`Matchers` can also be called asserts. They verify whether the execution of the method produced the result you expected.

~~~
expect(sayHello(name)).check.if.they.equal('Hello, Carol');
~~~

There are several types of matchers:

- Those that check for exact equality
- Those that check for inequality
- Whether an array or object contains an expected value

Among many other types of matchers.

### Setups and teardown/hooks

<p align="center">
  <img width="300px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/setup.gif" />
</p>

`Setups` are organizations of methods that will be used during test execution. They receive `hooks`.
Hooks are pieces of code run in response to some action. They serve to avoid duplicated code. They can be used to create methods that should run before each test, or to clear and reset suite variables before each execution.

Imagine that my `say-hello.js` file has a function called `sayHello` that receives a name and another function called `sayBye` that receives a name and says "Bye, {Name}".  
To avoid writing two variables with the name, I can use a hook to serve both and restore it after each use to prevent errors.


#### Hook Types

~~~
before(() => { runs once before the block });

after(() => { runs once after the block });

beforeEach(() => { runs once before EACH block });

afterEach(() => { runs once after EACH block });
~~~

Example:

~~~
// setup

beforeEach(() => {
 const name = 'Carol';
});

afterEach(() => {
 name.restore();
});

// specs

it('Expects to say hello when it receives a name', () => {
  expect(sayHello(name)).to.be.a('Hello, Carol');
});

it('Expects to say bye when it receives a name', () => {
 expect(sayBye(name)).to.be.a('Bye, Carol');
});
~~~
