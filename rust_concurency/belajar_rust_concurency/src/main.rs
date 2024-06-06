fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 1..=5 {
                println!("counter: {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });
        println!("Application finish");
        thread::sleep(Duration::from_secs(7))
    }

    #[test]
    fn test_join_handler() {
        let handle = thread::spawn(|| {
            let mut counter = 0;
            for i in 1..5 {
                println!("counter: {}", i);
                thread::sleep(Duration::from_secs(1));
                counter += i;
            }
            return counter;
        });
        let res = handle.join(); // disini akan blocking karena menunggu thread untuk selesai
        match res {
            Ok(counter) => println!("Counter: {}", counter),
            Err(error) => println!("error: {:?}", error),
        }
        println!("App finished")
    }
}
