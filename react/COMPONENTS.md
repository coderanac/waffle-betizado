# Components

Components are like little boxes where we can store small pieces of code so we can reuse them.
Imagine building an application with 5 pages, all with different content, but the same header and footer. In the old days we had to rewrite them in
HTML 5 times (once per HTML file). Given that problem, can you already see the advantage of components? We can write the header and footer just once and
then call them wherever they're needed.
Multiple components can form a complete piece of software because they connect through an interface — this communication between them is called composition.
Imagine a landing page made up of a component with the header, one with a form, and one with the footer: we have a composition of components forming a complete site.

## [Components in Angular](https://angular.io/guide/architecture-components)

- Every Angular project has at least one component
- They are created through Classes
- They are bound to a template
- Controls a piece of view, which is the smallest grouping of elements that can be created and destroyed together given a directive
- Must be declared in ngModules to be used in other parts of the project
- Must include the `@Component` decorator in its declaration to receive directives
- We can define directives for it such as:
  - Lifecycle
  - `selector` (css)
  - `templateUrl` (html)
  - `providers` (dependencies)
- The view is hierarchical
- Has two-way data binding with the template
- Are dynamic
- Are organized in a tree of dependent components

## [Components in Vue](https://br.vuejs.org/v2/guide/components.html)

- Can be defined with `Vue.component()` to be global in the application, or in a `.vue` file with template, script, and style
- Are reusable instances
- Accept:
  - Lifecycle
  - `data`
  - `computed`
  - `watch`
  - `methods`
- Can be reused infinitely
- You can use the same component more than once and add rules to affect only the component that received an interaction
- Are organized in a tree of nested components
- You can create them anywhere in your application
- Accepts as many props as you want
- Only accepts one root element
- Can be dynamic
- Can listen to children's events and also change them


## [Components in React](https://pt-br.reactjs.org/docs/components-and-props.html)

- Are JavaScript functions
- Receive props
- Return React elements
- Can be declared via Classes or Functions
- Can be dynamic
- Their behavior can be defined by props
- Can be composed of other components

# Summary

Web Components solve the code reuse problem and Frameworks and Libs use them in a way that goes beyond static code. Now, not only can you
use the Header on all 5 pages, you can also change the type of option displayed in each header menu depending on which page the user is on.
Each tool has its own way of declaring, organizing, and using components, children, building its tree, etc. But once you understand the concept and what it solves, it becomes
much easier to use it and get the most out of the feature.
