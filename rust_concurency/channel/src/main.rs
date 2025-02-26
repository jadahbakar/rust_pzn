fn main() {
    println!("Hello, Channel!");
}

// Channel -----------------------------------------------------------------------

#[cfg(test)]
mod test {
    use std::{
        sync::{
            atomic::{AtomicI32, Ordering},
            Arc,
        },
        thread,
        time::Duration,
    };
    #[test]
    fn test_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handler_1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            sender.send("Hello from thread".to_string())
        });

        let handler_2 = thread::spawn(move || {
            let message = receiver.recv().unwrap();
            println!("{}", message);
            // or ---
            // match receiver.recv() {
            //     Ok(message) => println!("{}", message),
            //     Err(err) => println!("Error message: {:?}", err),
            // }
            // or ---
            // let message = receiver.recv()?;
            // println!("{}", message);
        });

        let _ = handler_1.join();
        let _ = handler_2.join();
    }

    #[test]
    fn test_multi_sender() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let handler_1 = thread::spawn(move || {
            for i in 1..5 {
                thread::sleep(Duration::from_secs(1));
                let _ = sender.send(format!("data: {}", i));
            }
            let _ = sender.send("Exit".to_string());
        });

        let handler_2 = thread::spawn(move || loop {
            let message = receiver.recv().unwrap();
            if message == "Exit" {
                break;
            }
            println!("{}", message)
        });

        let _ = handler_1.join();
        let _ = handler_2.join();
    }

    #[test]
    fn test_iterator() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let handler_1 = thread::spawn(move || {
            for i in 1..5 {
                thread::sleep(Duration::from_secs(1));
                let _ = sender.send(format!("data: {}", i));
            }
        });

        let handler_2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("{}", value)
            }
        });

        let _ = handler_1.join();
        let _ = handler_2.join();
    }

    #[test]
    fn test_multi_sender_clone() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let sender2 = sender.clone();

        let handler_1 = thread::spawn(move || {
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(2));
                let _ = sender.send(format!("data: {}", i));
            }
        });

        let handler_3 = thread::spawn(move || {
            for i in 1..=5 {
                thread::sleep(Duration::from_secs(2));
                let _ = sender2.send(format!("data: {}", i + 5));
            }
        });

        let handler_2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("{}", value)
            }
        });

        let _ = handler_1.join();
        let _ = handler_2.join();
        let _ = handler_3.join();
    }

    // This will cause Race Condition ------------------------------
    static mut COUNTER: i32 = 0;
    #[test]
    fn race_condition() {
        let mut handlers = vec![];
        for _ in 0..=10 {
            let handler = thread::spawn(|| unsafe {
                for _j in 0..1000000 {
                    COUNTER += 1
                }
            });
            handlers.push(handler);
        }

        for hdlr in handlers {
            hdlr.join().unwrap()
        }

        println!("counter: {}", unsafe { COUNTER })
    }

    // Solved using Atomic ------------------------------
    // Atomic merupakan tipe data yang digunakan untuk sharing untuk beberapa thread
    // Atomic sendiri merupakan tipe data yang membungkus tipe data aslinya
    // Kita bisa pilih jenis tipe data Atomic, sesuai dengan tipe data aslinya yang akan kita gunakan

    #[test]
    fn test_atomic() {
        static COUNTER: AtomicI32 = AtomicI32::new(0);
        let mut handlers = vec![];

        for _ in 1..=10 {
            let handler = thread::spawn(move || {
                for _ in 1..=1000000 {
                    COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }

        for hdlr in handlers {
            hdlr.join().unwrap()
        }
        println!("counter: {}", COUNTER.load(Ordering::Relaxed));
    }

    // Atomic Reference - Atomic Reference Counted (Arc) ------------------------------
    // Tipe data yang bisa digunakan untuk membuat reference ke data lain,
    // Tipe ini mirip seperti tipe Rc (Reference counted),
    // Namun karena semua operasi Arc itu atomic,
    // Oleh karena itu operasinya lebih mahal tapi keuntungannya adalah thread safe
    #[test]
    fn test_atomic_reference() {
        let counter: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));
        let mut handlers = vec![];
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                for _ in 0..1000000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }

        for hdlr in handlers {
            hdlr.join().unwrap()
        }

        println!("counter: {}", counter.load(Ordering::Relaxed));
    }

    // Mutex (Mutual Exclusion) ------------------------------------------------------------
    // Tipe data yang digunakan untuk melindungi data yang di-sharing ke lebih dari satu thread
    // Mutex akan memblok thread dan menunggu sampai lock (kunci) tersedia
    // Method lock() pada Mutex untuk menunggu sampai mendapatkan data - setelah data keluar dari scope -
    // lock (kunci) akan dikembalikan ke Mutex - sehingga thread lain bisa mengambil lock (kunci) nya
    // -- ini cocok apabila ingin melindungi suatu tipe data yg tidak ada tipe data atomic nya
    // sehingga menggunakan mutext ini untuk melindungi saat akan memanipulasi datanya.
    // kunci == antri
}
