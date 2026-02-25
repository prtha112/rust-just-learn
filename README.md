# Rust Just Learn
link : https://google.github.io/comprehensive-rust/running-the-course/course-structure.html

## Mutable Borrowing

```rust
fn add_one(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut x = 1;
    add_one(&mut x);
    println!("{}", x);
}
```

## Immutable Borrowing

```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("hello");
    print_length(&s);
    print_length(&s);
}
```

## Struct

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("x: {}, y: {}", p.x, p.y);
}
```

## Enum

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let c = Color::Red;
    println!("Color: {}", c);
}
```

## Warning unwrap

```rust
let max_connection = env::var("MAX_CONNECTION")
        .expect("MAX_CONNECTIONS must be set")
        .parse::<u32>()
        .expect("MAX_CONNECTIONS must be a number"); // This is best practice

let max_connection = env::var("MAX_CONNECTION")
        .expect("MAX_CONNECTIONS must be set")
        .parse::<u32>()
        .unwrap(); // This is not best practice because it will panic if the value is not a number
```