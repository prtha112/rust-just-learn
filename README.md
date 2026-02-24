# Rust Just Learn
link : https://google.github.io/comprehensive-rust/running-the-course/course-structure.html

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