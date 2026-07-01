# Testing Tools

### Mocha

<p align="center">
 <img width="400px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/mocha.gif" />
</p>

With Mocha you can write `asynchronous` tests. It is a JavaScript code framework that can run on `node.js` and in `browsers`. Its execution is
synchronous.

### Chai

<p align="center">
 <img width="400px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/chai.gif" />
</p>

Chai is an assertion library that can be used with any JavaScript testing framework. It has a readable and clear interface.

Example of Mocha using Chai:

~~~
describe('sayHello', () => {
  it('Should say hello when it gets a name', () => {
    expect(sayHello('Carol')).to.be.a('Carol');
  });
});
~~~

### SinonJs

<p align="center">
 <img width="400px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/spies.gif" />
</p>

SinonJS is a library of spies, stubs, and mocks for unit tests. It works with any unit testing framework.

Example:

~~~
expectedName = { name: 'Carol' };

const db = sinon.mock(Database);

db.expects('save').once().withArgs(expectedName);

sayHello(expectedName, () => {});

db.verify();
db.restore();
~~~

### Jest

<p align="center">
 <img width="400px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/jest.gif" />
</p>

Jest is a testing framework for JavaScript. Unlike Mocha, it already ships with its own matchers, asserts, setups, mocks, and other resources needed to
develop complete tests. Mocha, on the other hand, requires external resources such as Chai, Sinon, and Nock.

Example:

~~~
test('sayHello', () => {
  expect(sayHello('Carol')).toBe('Hello, Carol');
});
~~~

### Jasmine

<p align="center">
 <img width="400px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/jasmine.gif" />
</p>

Jasmine is a testing framework that follows the BDD (behavior-driven) testing concept for testing JavaScript. It also has a very
clean and concise syntax.

Example:

~~~
describe('say hello', () => {
  it('Should say hello when it gets a name', () => {
    expect(sayHello()).toEqual('Hello!');
  });
});
~~~

### NockJs

<p align="center">
 <img width="400px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/nock.gif" />
</p>

NockJS is a NodeJS library that mocks HTTP requests. It is used to intercept a request and simulate a response.

Example:

~~~
import axios from 'axios';
import nock from 'nock';
import test from 'ava';

test('can fetch test response', async t => {
  // Set up the mock request.
  const scope = nock('http://localhost')
    .get('/test')
    .reply(200, 'test response')

  await axios.get('http://localhost/test')

  scope.done()
})
~~~

## End-to-End Tests

<p align="center">
 <img width="400px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/e2e.gif" />
</p>


### Selenium

Selenium is a tool for simulating the browser. With it you can run any type of test that requires a browser to
execute, such as end-to-end tests. On top of being extremely complete, it also supports multiple languages like C#, Java, Python, and Perl, and has
its own IDE.

### Protractor

Moving on to end-to-end (e2e) tests. Protractor is used to run UI tests — it simulates a user's navigation and records and measures the tests.
It is used to run e2e tests in the Angular framework.

### Cypress

Like Protractor, Cypress is used for end-to-end tests. Its main differences are that Cypress doesn't require Selenium — its entire architecture is
its own, aimed at improving test performance — and that Cypress runs on all front-end frameworks.

### Cucumber

Cucumber is a software for test development. It has its own language and provides test support for many programming languages: JavaScript, Java, Ruby,
PHP, Lua...
It is focused on BDD testing.

