struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
        }
    }

    fn profile(&self) -> String {
        format!("Your name is {}, you are {}.", &self.name, &self.age)
    }
}

fn main() {
    let alice = User::new(String::from("Alice"), 28);
    println!("{}", alice.profile());
}
