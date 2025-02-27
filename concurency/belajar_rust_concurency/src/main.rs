fn main() {
    println!("Hello, thread!");
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};
    use tokio::runtime::Runtime;

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
    // Untuk menampilkan output secara realtime di terminal gunakan -- --nocapture
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

    use std::{
        cell::RefCell,
        sync::{
            atomic::{AtomicI32, Ordering},
            Arc, Barrier, Mutex, Once,
        },
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
    #[test]
    fn test_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handlers = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                for _ in 0..1000000 {
                    let mut data = counter_clone.lock().unwrap();
                    *data += 1;
                }
                // data akan di unlock secara otomatis setelah keluar dari scope
            });
            handlers.push(handler);
        }

        for h in handlers {
            h.join().unwrap()
        }

        println!("counter {}", *counter.lock().unwrap())
    }

    // Thread local ------------------------------------------------------------
    // menyimpan data di dalam Thread
    // misal membuat data di dalam scope thread selama thread tersebut aktif dan tidak bertukar dengan thread lain
    // macro = thread_local!
    // Cell - mutable / RefCel - imutable

    thread_local! {
        pub static NAME: RefCell<String> = RefCell::new("Default".to_string());
    }

    #[test]
    fn test_thread_local() {
        let handler = thread::spawn(|| {
            NAME.with_borrow_mut(|name| *name = "Budi".to_string());
            NAME.with_borrow(|name| println!("Hello: {}", name));
        });
        handler.join().unwrap();

        NAME.with_borrow(|name| {
            println!("Hello, {}", name);
        })
    }

    // Thread Panic ------------------------------------------------------------
    /*
    Apa akibatnya ketika terjadi panic di dalam thread?
    Maka thread tersebut akan berhenti, tapi tidak akan menghentikan thread lainnya
    Jadi tidak perlu khawatir ketika menjalankan thread baru, dan terjadi panic pada thread tersebut,
    maka thread utama (main) tidak akan berhenti, karena berbeda thread
    Kecuali jika terjadi panic di thread utama (main), otomatis thread utama akan berhenti
    */

    #[test]
    fn test_thread_panic() {
        let handle = thread::spawn(|| {
            panic!("something when wrong 1");
        });

        match handle.join() {
            Ok(_) => println!("thread finish"),
            Err(_) => println!("thread panic"),
        }

        println!("Application finish");
    }

    // Barrier ------------------------------------------------------------
    /*
    Barrier merupakan tipe data yang bisa digunakan agar beberapa thread menunggu sebelum melakukan pekerjaannya secara bersamaan
    Contoh misal, kita akan membuat kode program yang menunggu jika 10 thread sudah ada,
    baru semuanya boleh berjalan, jika belum 10 thread, maka program tidak boleh berjalan terlebih dahulu
    */
    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(10));
        let mut handlers = vec![];

        for i in 0..10 {
            let barrier_clone = Arc::clone(&barrier);
            let handler = thread::spawn(move || {
                println!("Join Gamer: {}", i);
                barrier_clone.wait();
                println!("Gamer Start: {}", i);
            });
            handlers.push(handler);
        }

        for h in handlers {
            h.join().unwrap()
        }
    }

    // Once ------------------------------------------------------------
    /*
    variable yang perlu diinisialisasi datanya diawal cukup sekali saja
    memastikan bahwa hanya ada satu thread yang bisa memanggil proses inisialisasi datanya
    menjaga bahwa hanya ada satu thread saja yang bisa memanggil proses inisialisasi, dan hanya sekali saja dipanggil
    */

    static mut TOTAL_COUNTER: i32 = 0;
    static TOTAL_INIT: Once = Once::new();

    fn get_total() -> i32 {
        unsafe {
            TOTAL_INIT.call_once(|| {
                println!("Call Once");
                TOTAL_COUNTER += 1;
            });
            TOTAL_COUNTER
        }
    }

    #[test]
    fn test_once() {
        let mut handlers = vec![];
        for _ in 0..10 {
            let handler = thread::spawn(|| {
                let total = get_total();
                println!("Total: {}", total);
            });
            handlers.push(handler);
        }

        for h in handlers {
            h.join().unwrap()
        }
    }

    // Future ------------------------------------------------------------
    /*
    Future adalah representasi dari komputasi asynchronous
    Future merupakan value yang memungkinkan komputasinya belum selesai.
    Dengan menggunakan Future, memungkinkan thread untuk melanjutkan pekerjaan lainnya,
    selama menunggu nilainya ada pada Future
    */
    /*
    Future memiliki satu method bernama poll(), yang digunakan untuk mengambil data di Future
    Hasil dari poll() method adalah data enum Poll
    Pada enum Poll, terdapat dua opsi, Ready jika data sudah ada, Pending jika data belum tersedia
    */
    // membuat Future
    /*
    Future merupakan Trait, untuk membuat Future, kita perlu menggunakan method dengan kata kunci async
    Method dengan kata kunci async, secara otomatis datanya akan mengembalikan tipe data Future
    */

    // Async  ------------------------------------------------------------
    // Function yang menggunakan kata kunci async, maka return value nya adalah Future
    async fn get_async_data() -> String {
        thread::sleep(Duration::from_secs(2));
        println!("message from async");
        return "Hello from async".to_string();
    }
    /*
    Kode async tidak bisa dipanggil pada kode non async, oleh karena itu untuk memanggil kode async, kita harus menggunakan kode async
    Sayangnya, secara default Rust hanya menyediakan kontrak untuk membuat kode async, ketika ingin menjalankan kode async, kita perlu menggunakan Runtime / Executor, dan secara default Rust tidak menyediakan
    Oleh karena itu, kita perlu menggunakan library Runtime / Executor untuk menjalankan kode async
    */
    /*
    Untuk melakukan pengetesan kode Async, kita bisa menggunakan Tokio
    Hal ini karena secara default Rust tidak mendukung unit test kode async
    Kita bisa menggunakan attribute tokio::test
    */
    #[tokio::test]
    async fn test_async() {
        get_async_data();
    }
    // Await ------------------------------------------------------------
    /*
    Secara default, Future merupakan tipe data Lazy, artinya tidak akan dieksekusi jika tidak dijalankan
    Agar Future dieksekusi, kita bisa menggunakan await
    Await hanya bisa digunakan dalam kode async, karena yang dilakukan await sebenarnya adalah melakukan poll() terhadap Future, berbeda dengan join() pada Thread
    */
    #[tokio::test]
    async fn test_async_await() {
        let funcion = get_async_data();
        println!("Finish Call Async");
        let data = funcion.await;
        println!("{:#?}", data);
    }

    // TASK ------------------------------------------------------------
    // lightweight thread
    // CONCURENT ------------------------------------------------------------
    /*
    Task adalah implementasi dari Concurrent,
        dimana jika kita menggunakan Thread,
        Thread tidak bisa berpindah-pindah pekerjaan,
        harus menyelesaikan pekerjaan sampai selesai
    Sedangkan Task, sebenarnya secara internal,
        Task tetap akan dijalankan dalam Thread,
        namun Thread yang menjalankan Task, bisa berpindah-pindah Task sesuai kebutuhan,
        misal ketika kita menghentikan Task dengan sleep(), Thread akan menjalankan Task yang lainnya
    */

    async fn get_database_data(wait: u64) -> String {
        println!("{:?}: get database data", thread::current().id());
        tokio::time::sleep(Duration::from_secs(wait)).await;
        println!("{:?}: hello from database", thread::current().id());
        return "Hello from db".to_string();
    }

    #[tokio::test]
    async fn test_concurrent() {
        let mut handlers = vec![];
        for i in 0..5 {
            let handler = tokio::spawn(get_database_data(i));
            handlers.push(handler);
        }

        for h in handlers {
            let data = h.await.unwrap();
            println!("{}", data);
        }
    }

    // TASK RUNTIME ------------------------------------------------------------
    async fn run_concurent(runtime: Arc<Runtime>) {
        let mut handlers = vec![];
        for i in 0..5 {
            let handler = runtime.spawn(get_database_data(i));
            handlers.push(handler);
        }

        for h in handlers {
            let data = h.await.unwrap();
            println!("{}", data)
        }
    }

    #[test]
    fn test_runtime() {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(10)
                .enable_time()
                .build()
                .unwrap(),
        );
        // menjalankan async function di dalam runtime
        runtime.block_on(run_concurent(Arc::clone(&runtime)))
    }
}
