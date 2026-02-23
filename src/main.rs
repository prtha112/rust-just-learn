struct User {
    id: u32,
    name: String,
    active: bool,
}

enum JobState {
    Pending,
    Running,
    Failed(String),
    Success,
}

enum DeploymentState {
    Creating,
    Ready,
    Error(String),
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

impl JobState {
    fn describe(&self) -> String {
        match self {
            JobState::Pending => "Pending".to_string(),
            JobState::Running => "Running".to_string(),
            JobState::Failed(err) => format!("Failed: {}", err),
            JobState::Success => "Success".to_string(),
        }
    }
}

impl DeploymentState {
    fn can_serve(&self) -> bool {
        matches!(self, DeploymentState::Ready)
    }

    fn message(&self) -> String {
        match self {
            DeploymentState::Creating => "creating".into(),
            DeploymentState::Ready => "ready".into(),
            DeploymentState::Error(e) => format!("error {}", e),
        }
    }
}

trait Speak {
    type Err;

    fn speak(&self) -> Result<String, Self::Err>;
    fn shout(&self) -> Result<String, Self::Err> {
        Ok(self.speak()?.to_uppercase())
    }
}

impl Speak for User { // impl trait for struct
    type Err = ();

    fn speak(&self) -> Result<String, Self::Err> {
        Ok(format!("Hello {}", self.name))
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

    let state = JobState::Pending;
    println!("{}", state.describe());
    let state = JobState::Running;
    println!("{}", state.describe());
    let state = JobState::Failed("timeout".to_string());
    println!("{}", state.describe());
    let state = JobState::Success;
    println!("{}", state.describe());

    let dep = DeploymentState::Creating;
    println!("Can serve: {}", dep.can_serve());
    println!("Message: {}", dep.message());

    let dep = DeploymentState::Ready;
    println!("Can serve: {}", dep.can_serve());
    println!("Message: {}", dep.message());

    let dep = DeploymentState::Error("timeout".to_string());
    println!("Can serve: {}", dep.can_serve());
    println!("Message: {}", dep.message());

    let user = User {
        id: 3,
        name: String::from("Hanni"),
        active: true,
    };
    match user.speak() {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("speak error: {:?}", e),
    }
    match user.shout() {
        Ok(msg) => println!("{}", msg),
        Err(_) => println!("error"),
    }
}

fn calculate_length(s: &String) -> usize { // immutable borrow
    s.len()
}

fn change(s: &mut String) { // mutable borrow
    s.push_str(" world 3");
}