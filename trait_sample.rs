use std::fmt::{self, Display};

struct User {
    name: String,
    age: u32,
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Your are {}, your age is {}", &self.name, &self.age)
    }
}

impl User {
    fn new (name: String, age: u32) -> Self {
        User { name, age }
    }
}

fn main() {
    let alice = User::new("alice".to_string(), 28);
    println!("{}", alice);
}
