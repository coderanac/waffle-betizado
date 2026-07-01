# Virtual DOM
The Virtual DOM was created to make rendering to the Real DOM more performant. It operates in 3 steps:

In the first step it creates the Real DOM, then it makes a virtual copy of it, and finally it applies the updates.  
In other words, it keeps a virtual copy of the Real DOM in memory, and from there it identifies changes that occur in layout, CSS, or actions and updates only what changed.
This change in turn does not trigger a re-render in the Real DOM, because the Virtual DOM processes the change and then reflects it in the Real DOM.
