use crate::third::say_third;

pub fn say_first() {
    println!("Hello from first module");
    say_third();
}

pub mod fourth {
    pub mod fifth {
        pub fn say_fifth() {
            super::super::say_first();
        }
    }
}
