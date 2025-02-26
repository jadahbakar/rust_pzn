fn main() {
    println!("Hello, thread!");
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use std::{result, thread};

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
    // untuk menampilkan output secara realtime di terminal gunakan -- --nocapture
    // cargo test tests::test_create_thread  -- --nocapture

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

    fn calculate() -> i32 {
        let mut counter = 0;
        for i in 0..=5 {
            println!("Counter: {}", i);
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
        return counter;
    }

    #[test]
    fn test_sequential() {
        let result_1 = calculate();
        let result_2 = calculate();
        println!("App finished")
    }

    #[test]
    fn test_parallel() {
        let handle_1 = thread::spawn(|| calculate());
        let handle_2 = thread::spawn(|| calculate());

        let result_1 = handle_1.join();
        let result_2 = handle_2.join();

        match result_1 {
            Ok(counter) => println!("Total 1: {}", counter),
            Err(error) => println!("Error: {:?}", error),
        }

        match result_2 {
            Ok(counter) => println!("Total 1: {}", counter),
            Err(error) => println!("Error: {:?}", error),
        }
        println!("App finished")
    }

    #[test]
    fn test_closure() {
        let name = String::from("Dedy");
        let closure = || {
            thread::sleep(Duration::from_millis(1));
            println!("Hello, {}", name)
        };

        closure();
    }

    #[test]
    fn test_closure_moving_ownership() {
        let name = String::from("xxx");
        //  ^^ may outlive borrowed value `name`
        // let closure = || {
        //     thread::sleep(Duration::from_millis(2));
        //     println!("hello, {}", name)
        // };

        let closure = move || {
            thread::sleep(Duration::from_millis(2));
            println!("hello, {}", name)
        };

        let handler = thread::spawn(closure);
        handler.join().unwrap();

        // borrow of moved value: `name`
        // error karena var name sudah berpindah ke closure
        // println!("call name again: {}", name);
    }

    #[test]
    fn test_current_thread() {
        let current_thread = thread::current();
        println!("{:#?}", current_thread.name().unwrap());
    }

    fn current_calc() -> i32 {
        let mut counter = 0;
        let current = thread::current();
        for i in 0..=5 {
            match current.name() {
                Some(name) => {
                    println!("{:?} - Counter: {}", name, i)
                }
                None => {
                    println!("{:?} - Counter: {}", current.id(), i)
                }
            }
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
        return counter;
    }

    #[test]
    fn test_parallel_current() {
        let handle_1 = thread::spawn(|| current_calc());
        let handle_2 = thread::spawn(|| current_calc());

        let result_1 = handle_1.join();
        let result_2 = handle_2.join();

        match result_1 {
            Ok(counter) => println!("Total 1: {}", counter),
            Err(error) => println!("Error: {:?}", error),
        }

        match result_2 {
            Ok(counter) => println!("Total 2: {}", counter),
            Err(error) => println!("Error: {:?}", error),
        }
        println!("App finished")
    }

    // Thread Factory ----------------------------------------------------------------
    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("My thread".to_string());

        let handler = factory
            .spawn(current_calc)
            .expect("Failed to create a new thread");
        let total = handler.join().unwrap();
        println!("total counter: {}", total);
    }
}
