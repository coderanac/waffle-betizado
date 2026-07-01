# CSS-IN-JS

## A brief history

CSS-in-JS was first introduced at [NationJS in 2014 by Christopher Chedeau (vjeux)](https://speakerdeck.com/vjeux/react-css-in-js). He proposed
a solution to problems such as cross-class conflicts, "dead" code that was painted but should no longer be rendered, reduction of code repetition, reduction of global classes,
among other problems with using plain CSS in React.  
Angular already handled encapsulated and scoped CSS for modules and pages very well. VueJS also brought a native scoped CSS solution out of the box.
But until then React still struggled a lot with these issues and solutions like plain CSS were not enough. Because you could have this in more than one place in the project:

```
.form-button__submit {
  ...
}
```

So given the concept, libraries emerged that offered the feature. And now we no longer had to worry about having more than one ```.menu```, because as long as they were in separate
files, it would be transformed into something like: ```<p class="menu_746732">...</>```. On top of the wild hashes, css-in-js libs managed styles on demand —
if they were needed they were rendered, if not, they were removed from the final document. And the management of isolation and sharing became much simpler.

## But why CSS with Javascript?

Vjeux's idea was to keep development simple and readable for developers. So merging CSS into JavaScript syntax and turning everything into a single concern
was the idea to solve the problem in a less exhausting way, with a smaller learning curve. So the CSS that used to look like this:

```
<style>
  .button {
    color: red;
  }
</style>
```

Became this:

```
const Box = style.div(
  {
    color: 'blue',
  }
);
```

## Pros and cons:

### Pros

- Dynamic style management tied to component lifecycle
- Scoped styles without needing to configure rollup or webpack
- Reduction of code repetition
- Smart variable sharing
- Reduction of dependencies
- Easy scalability

### Cons
- Final class names are strange hashes, which makes it harder to track down the original class

## Recommendations

A lib for those working with Javascript: [Styled Component](https://styled-components.com/).  
A lib for those working with Typescript: [TypeStyle](https://github.com/typestyle/typestyle).
