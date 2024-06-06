pub struct User {
    pub name: String,
    pub id: u8,
}

impl User {
    pub fn sayHello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.name)
    }
}
