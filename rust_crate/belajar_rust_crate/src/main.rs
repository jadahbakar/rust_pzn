use belajar_rust_crate_say_hello::{get_time, say_greetings};
use say_hello_styawan::{say_goodbye, say_greeting, say_hello};

fn main() {
    println!("{}", say_hello("dedy"));
    println!("{}", say_goodbye("dedy"));
    println!("{}", say_greeting("dedy"));

    println!(
        "Hi, Good {}, right now is : {}",
        say_greetings(),
        get_time()
    );
}
