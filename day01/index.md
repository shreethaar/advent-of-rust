# Day 01

This is the challenge code:

```rust

pub fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(gift_message);

    println!("{}", gift_message);
}

pub fn attach_message_to_present(message: String) {
    println!("The present now has this message: {}", message);
}
```

The first challenge test on the concept of ownership and borrowing rules. 

In the code:
```rust
attach_message_to_present(gift_message);
```

When we pass `gift_message` to `attach_message_to_present`, the ownership of the `String` was moved to the function. This is because of `String` does not implement the `Copy` trait. In Rust, the `copy` trait happen implicitly and is not overloadable. The `Copy` trait is only implement for types that:
- Do not require heap allocation
- Do not implement the `Drop` trait, as `Drop` implies that special cleanip is required.

To fix the code, there are two approaches:

1. Borrow with a reference

```rust
pub fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(&gift_message);

    println!("{}", gift_message); // gift_message is still valid
}

pub fn attach_message_to_present(message: &String) {
    println!("The present now has this message: {}", message);
}
```

2. Clone the string

```rust
pub fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(gift_message.clone());

    println!("{}", gift_message); // gift_message is still valid
}

pub fn attach_message_to_present(message: String) {
    println!("The present now has this message: {}", message);
}
```

For the sake of efficiency, I choose the first solution unless cloning the `gift_message` is explicitly required.
