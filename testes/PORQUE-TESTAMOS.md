# Why test the application?

<p align="center">
  <img src="https://github.com/coderanac/waffle-betizado/blob/master/assets-waffles/pq-testar.gif" />
</p>

## Quality:

Testing allows us to confirm characteristics that define a quality application.

- Application
  - Does what it's supposed to do
  - Adds value
  - Is reliable and secure
  - Is performant
- The code
  - Is easy to understand
  - Is adaptive and scalable
  - Is reusable
  - Is maintainable
  - Is testable

## Code review

Whether with TDD or normal tests, whenever we write tests we tend to review, think, and ensure quality.

- Semantic names
- Small methods
- Methods with a single responsibility
- Methods that are easy to test
- Methods without comments: readable code doesn't need comments
- Avoid technical debt

## Why test?

- Reduce analysis time
- Find bugs
- Refactoring
- Generates a description of each method **
- Reduces redundant code
- Ensures quality

** this way there's no need to re-read the entire application, making maintenance and onboarding of other devs on the project easier.

## Conclusion

As developers we should care about delivering concise, secure applications that add value and don't carry technical debt.
Tests are one way to guarantee this. Tested code is reviewed, refactored, free of bugs, and in this way we guarantee an application that delivers real value.  
Well-written tests generate value not only for the client, but also for the development team, which receives a kind of "documentation" of the application: "Method X should
do this" — on top of finding readable code.  
And it's always good to remember: 100% test coverage doesn't mean good tests.
