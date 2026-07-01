## Spies, Stubs, Mocks, and Fakes

### Spies

<p align="center">
  <img width="450px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/spies.gif" />
</p>

They spy on whether the expected execution occurred — they check whether a method was called, how it was called, the arguments it received, and what it returned. We use them to
test callbacks and how methods are used within the system. For example: testing console.log.

An example to check whether the `sayHello` function was called once:

~~~
const callback = sinon.spy();

sayHello("Carol", callback);

assertTrue(callback.calledOnce);
~~~

### Stubs

<p align="center">
  <img width="450px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/stubs.gif" />
</p>

Stubs are very similar to spies, but in addition to watching the method, you can also replace its behavior. We can use them to force an
exception case, to skip a part of the function that isn't needed for the test, or to simplify an asynchronous test.

Imagine that the `sayHello` function receives an object with names and I want to replace the value of the first name to check whether my method is working as expected.

~~~
sinon.stub(names, 'name1').get(() => 'Carol');

names.name1 // return 'Carol'
~~~

### Mocks

<p align="center">
  <img width="450px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/mocks.gif" />
</p>

Mocks are used to fake both input and output values, change the method's behavior, and pre-program responses and expectations. In other words, with them
we can modify everything about a method. We use them when we need a Stub, but also need to verify multiple behaviors of our method.

Still with the `sayHello` function, imagine that in addition to building the phrase it also saves the name to the Database. I want to test whether it is saving once to the
database, but I don't want to actually save my test data to the DB, so I'll mock it.

It's important to always restore the mocked variables at the end of the mock.

~~~
expectedName = { name: 'Carol' };

const db = sinon.mock(Database);

db.expects('save').once().withArgs(expectedName);

sayHello(expectedName, () => {});

db.verify();
db.restore();
~~~

### Fakes

<p align="center">
  <img width="450px" src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/fake.gif" />
</p>

And finally: fakes.

Fakes are nothing more than false values created to check whether a test works. No support library is needed for them.

Still with `sayHello`, now it should receive a string and return "Hello, {Name}". My fake will be the const name:

~~~
it('should say hello when you receive a name', () => {
  const name = 'Carol';

  expect(sayHello(name)).to.deep.a('Carol');
});
~~~
