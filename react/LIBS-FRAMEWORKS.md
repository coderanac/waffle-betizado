# Libs and Frameworks

<p align="center">
  <img src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/framework-vs-lib.jpeg" />
</p>

Libraries are tools that solve specific problems, whether it's needing to work with interactive data visualization where we can use D3.js,
or even creating an SPA where we can use React.
Frameworks, on the other hand, offer a complete solution for application development, mostly with a very well-defined architecture design and their
own tools for end-to-end development included. And unlike libs, which can be used in a small part of the application, to use
a framework you will have to build your entire application on top of it.

## Which one is better?

Fortunately there is no right answer to that, which leaves you quite free to experience each of them and see which one solves your problem best.
Some things to consider:

- Architecture  
I'll use Angular as an example — it already defines a structure of best practices and architecture design, because its goal is to make web
development easier.
With a lib you have more freedom to implement whatever architecture you want. In React's own documentation they say they have no opinion on the architecture
you choose to use.

- Dependencies  
Angular already includes built-in dependency injection, while React requires us to install external solutions to complement it.

## But which one should I use?

If you need a robust solution, without many external dependencies, with best practices already defined, we can go with Angular (or another Framework).
If we're building a smaller application and looking for high performance, React might be the better option.
Another point to consider is the learning curve. Because Angular already has many built-in conventions, beyond learning to use it, we also need to learn
its architecture, best practice patterns, and other things that make the curve steeper — whereas in React the curve is much smaller and more intuitive, since we basically just need
to learn how to use its own features.
