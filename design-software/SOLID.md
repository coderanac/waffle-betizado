# SOLID

SOLID is a set of 5 principles of Object-Oriented Programming ([OOP](https://en.wikipedia.org/wiki/Object-oriented_programming)).
It was created to help us organize and maintain our code, making it more focused and less prone to errors.
<div style="align: center">
  <img src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/Solid.png" />
</div>

### S (SRP - Single Responsibility Principle)

<img src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/single.gif" />

This is the single responsibility principle — a class should have only one responsibility and a single reason to exist. For example:
If I have a bank and in it I store who my client is and what their card is, I can have one class that creates the client and one for the card, but if a single class
is responsible for both of these jobs that is wrong according to SOLID, because we lose clean code.

### O (OCP - Open-Closed Principle)

<img src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/open-close.gif" />

This principle states that objects and entities should be open for extension and closed for modification. Whenever there is something new to be added, we should
extend rather than modify the source code. This prevents bugs in code that is already working.

*** in some cases we can replace the method with an interface.

<img src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/soup-letter.gif" />

### L (LSP - Liskov Substitution Principle)

"Derived classes must be substitutable for their base classes."

The LSP warns us about the use of inheritance and is directly linked to the OCP.

### I (ISP - Interface Segregation Principle)

The interface segregation principle states that a class should not have interfaces it won't use. It is better to have a specific interface than a generic one that
won't be fully used.

### D (DIP - Dependency Inversion Principle)

The dependency inversion principle states that the application should depend on abstractions and not on implementations. Reduce class coupling.
