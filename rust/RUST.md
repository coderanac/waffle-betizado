# Rust 🦀

<img width="475" height="356" alt="source" src="https://github.com/user-attachments/assets/015938e1-fe21-4c18-9c7e-7d644ef1795e" />

Rust is a systems programming language that is obsessed with two things: **speed** and **safety**. Like that one friend who refuses to leave the house without sunscreen AND a helmet. Annoying at first, but honestly? They're never wrong.

It was created at Mozilla and has been voted **the most loved programming language** on Stack Overflow surveys for years in a row. Which means a lot of developers tried it, got yelled at by the compiler, and still came back for more. That says something.

---

## What problems does Rust solve?

<img width="500" height="282" alt="146413" src="https://github.com/user-attachments/assets/e3442f19-5f9c-47aa-905e-b3d9fdd631ef" />


Languages like C and C++ are extremely fast, but they let you do things like accidentally access memory that doesn't belong to you anymore — which leads to crashes, security vulnerabilities, and a bad time in general.

Languages like Python and JavaScript are safe and easy, but they're slower because they rely on a garbage collector to clean up memory for you at runtime.

Rust sits right in the middle: **as fast as C, as safe as... well, Rust**. It enforces memory safety at compile time, so there's no garbage collector and no surprises at runtime. The compiler is the strict one here so your production environment doesn't have to be.

---

## Ownership — The Most Famous Concept in Rust

This is the big one. Rust manages memory through a system called **ownership**, and it has three rules:

1. Each value has one **owner**
2. There can only be **one owner at a time**
3. When the owner goes out of scope, the value is **dropped** (freed from memory)

Think of it like a library book. Only one person can have it checked out at a time. When you return it (go out of scope), it's gone from your hands. Simple, right? The compiler makes sure nobody forgets to return the book.

```rust
fn main() {
    let book = String::from("The Rust Programming Language");
    let someone_else = book; // ownership moved here

    println!("{}", book); // 💥 compiler error! book was moved
}
```

Yeah. The compiler will not let that slide.

---

## Borrowing — Sharing Without Drama

<img width="500" height="300" alt="d55b3ba0ab3f507231d29ec8092c6b59" src="https://github.com/user-attachments/assets/9b130b33-f124-40a0-82ba-4f95ccd6083d" />

Sometimes you want to use a value without taking ownership of it. That's called **borrowing**, and you do it with the `&` symbol.

```rust
fn main() {
    let book = String::from("The Rust Programming Language");
    read(&book); // lending the book, not giving it away

    println!("Still have it: {}", book); // works fine ✅
}

fn read(b: &String) {
    println!("Reading: {}", b);
}
```

You can have **multiple read-only borrows** at the same time, but only **one mutable borrow** at a time. No two people writing in the same book simultaneously. The compiler enforces this so you never have race conditions.

---

## The Compiler Is (Annoyingly) Your Best Friend

<img width="220" height="221" alt="this-is-just-tough-love-man-anny-cho" src="https://github.com/user-attachments/assets/67936d83-7543-47a9-85af-9f658b509511" />

Here's the thing about Rust: the compiler error messages are actually... helpful? Like, genuinely helpful. It'll tell you what went wrong, why it went wrong, and often suggest exactly how to fix it.

```
error[E0382]: borrow of moved value: `book`
  --> src/main.rs:5:20
   |
 2 |     let book = String::from("...");
 3 |     let someone_else = book;
   |                        ---- value moved here
 4 |
 5 |     println!("{}", book);
   |                    ^^^^ value borrowed here after move
```

You'll spend more time fighting the compiler upfront, but your code will run correctly the first time it compiles. That's the trade.

---

## Basic Syntax

<img width="220" height="166" alt="get-things-done-getting-things-done" src="https://github.com/user-attachments/assets/c0646a65-2bad-4eeb-b161-61480d67e1ea" />

<img width="220" height="161" alt="spongebob-think" src="https://github.com/user-attachments/assets/d158bd12-db0c-46b7-9fda-645c12d9d145" />

### Variables

Variables are **immutable by default** in Rust. If you want to change a value, you have to explicitly say so with `mut`.

```rust
let x = 5;        // immutable — can't change this
let mut y = 10;   // mutable — go ahead
y = 20;           // works ✅
```

### Functions

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name) // no semicolon = this is the return value
}
```

### Structs

Structs are Rust's way of grouping related data together. No classes here.

```rust
struct Cat {
    name: String,
    lives: u8,
}

fn main() {
    let ferris = Cat {
        name: String::from("Ferris"),
        lives: 9,
    };

    println!("{} has {} lives left.", ferris.name, ferris.lives);
}
```

### Enums and Pattern Matching

One of the most satisfying features in Rust. The `match` expression forces you to handle every possible case — no forgotten edge cases.

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn what_to_do(light: TrafficLight) {
    match light {
        TrafficLight::Red    => println!("Stop!"),
        TrafficLight::Yellow => println!("...maybe stop?"),
        TrafficLight::Green  => println!("Go!"),
    }
}
```

---

## Option — No More Null Surprises

<img width="470" height="376" alt="giphy-downsized" src="https://github.com/user-attachments/assets/91d48c18-b459-44cf-919e-ad9b4baa3c4f" />


Rust doesn't have `null`. Instead, it has `Option<T>`, which forces you to deal with the possibility of something not existing.

```rust
fn find_item(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Rubber Duck"))
    } else {
        None
    }
}

fn main() {
    match find_item(1) {
        Some(item) => println!("Found: {}", item),
        None       => println!("Nothing here."),
    }
}
```

No more `Cannot read properties of null`. You're welcome.

---

## Result — Errors You Can't Ignore

<img width="311" height="177" alt="giphy" src="https://github.com/user-attachments/assets/563de7a7-e45a-4620-8fc9-c81b055df528" />


Error handling in Rust uses `Result<T, E>` instead of try/catch. Your function either succeeds (`Ok`) or fails (`Err`), and the caller has to handle both.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Can't divide by zero!"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e)     => println!("Oops: {}", e),
    }
}
```

---

## Should I learn Rust?

<img width="220" height="161" alt="spongebob-think" src="https://github.com/user-attachments/assets/6dbc1276-597a-4a76-9b2f-df43aa8bc22e" />


If you want to understand how memory actually works, write code that is genuinely fast and safe, or just flex at parties — yes.

If you're in a hurry to ship a web app this week — maybe not right now.

Rust has a steep learning curve, but every concept it teaches you will make you a better developer in any language. The ownership model alone will change the way you think about data.

Plus, the mascot is a crab named Ferris. That alone makes it worth it. 🦀
