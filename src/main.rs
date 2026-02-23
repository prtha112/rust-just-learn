struct User {
    id: u32,
    name: String,
    active: bool,
}

impl User { // Struct method
    fn new(id: u32, name: String) -> Self { // Associated function (constructor style)
        Self {
            id,
            name,
            active: true,
        }
    }
    fn greet(&self) { // immutable borrow
        println!("Hello {}", self.name);
    }
}

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // immutable borrow
    let r2 = &s; // immutable borrow
    println!("{} and {}", r1, r2);

    let r3 = &mut s; // mutable borrow
    r3.push_str(" world");
    println!("{}", r3);

    let r4 = r3.clone(); // clone s 
    println!("{}", r4);
    let len = calculate_length(&r4);
    println!("The length of '{}' is {}.", r4, len);

    let mut r5 = r4.clone(); // clone s
    change(&mut r5);
    println!("{}", r5);

    let user = User {
        id: 1,
        name: String::from("Tha"),
        active: true,
    };
    println!("{} is {} years old and active is {}", user.name, user.id, user.active);
    user.greet();

    let user2 = User::new(2, String::from("Karina"));
    println!("{} is {} years old and active is {}", user2.name, user2.id, user2.active);
    user2.greet();
}

fn calculate_length(s: &String) -> usize { // immutable borrow
    s.len()
}

fn change(s: &mut String) { // mutable borrow
    s.push_str(" world 3");
}