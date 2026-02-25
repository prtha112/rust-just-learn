# Rust Just Learn
link : https://google.github.io/comprehensive-rust/running-the-course/course-structure.html

## Ownership

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    println!("{}", s2);
}
```

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

## Implement Trait

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let p = Point::new(1, 2);
    println!("x: {}, y: {}", p.x, p.y);
}
```

## Trait

```rust
trait Speak {
    fn speak(&self) -> String;
}

struct Person {
    name: String,
}

impl Speak for Person {
    fn speak(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}

fn main() {
    let p = Person{ 
        name: "John".to_string() 
    };
    println!("{}", p.speak());
}
```

## Trait with default method

```rust
trait Speak {
    fn speak(&self) -> String;
    fn shout(&self) -> String {
        format!("{}", self.speak().to_uppercase())
    }
}

struct Person {
    name: String,
}

impl Speak for Person {
    fn speak(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}

fn main() {
    let p = Person {
        name: "John".to_string() 
    };
    println!("{}", p.speak());
    println!("{}", p.shout());
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