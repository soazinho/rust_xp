# 4. Programming Concepts

## Shadowing
- Shadowing
  - Compile-time error if we accidentally try to reassign to this variable without using the let keyword
  - Creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name

## Functions
- Statements vs Expressions
  - If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
  - The let y = 6 statement does not return a value, so there isn’t anything for x to bind to. This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

```rust
fn main() {
    let x = (let y = 6);
}
```
