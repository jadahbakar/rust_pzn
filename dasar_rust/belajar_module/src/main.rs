// mod model {
//     pub struct User {
//         pub name: String,
//         pub id: u8,
//     }

//     impl User {
//         pub fn sayHello(&self, name: &str) {
//             println!("Hello {}, my name is {}", name, self.name)
//         }
//     }
// }

// mod first {
//     pub fn say_hello() {
//         println!("Hello from first module")
//     }
// }

// mod second {
//     pub fn say_hello() {
//         println!("Hello form second")
//     }
// }

fn main() {}

// #[test]
// fn test_module() {
//     let user = model::User {
//         name: String::from("dedy"),
//         id: 10,
//     };
//     user.sayHello("Budi");
// }

// #[test]
// fn test_use() {
// use first::say_hello;
// use second::say_hello as say_hello_second;

// say_hello();
// say_hello_second();
// }
mod first;
mod model;
mod second;
mod third;

use first::say_first;
use model::User;
use second::say_second;

#[test]
fn test_use() {
    say_first();
    say_second();
    first::fourth::fifth::say_fifth();

    let user = User {
        name: String::from("dedy"),
        id: 10,
    };
    user.sayHello("Budi");
}

// CRATE
// CRATE adalah kode yang dijalankan oleh Rust Compiler untuk membuat aplikasi atau library
// crate src/main.rs
//
