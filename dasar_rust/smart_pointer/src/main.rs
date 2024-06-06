fn main() {
    println!("Hello, world!");
}

/*
> Pointer adalah konsep yang umum dimana sebuah variable berisi alamat lokasi data di memory
> Di Rust, reference merupakan pointer
> Smart Pointer adalah tipe data pointer namun memiliki metadata (informasi tambahan) dan kemampuan lain selain sebagai penunjuk ke lokasi data
> Di Rust yang menggunakan konsep ownership (pemilik) dan borrowing (meminjam),
    pada kebanyakan kasus,
    reference hanya meminjam data,
    sedangkan smart pointer merupakan pemilik dari data yang ditunjuk
> Menggunakan Box<T>, mengizinkan kita membuat data di Heap sedangkan pointer-nya disimpan di Stack

> Single data dari Box mungkin terlihat tidak begitu menarik,
        namun Box akan sangat berguna ketika kita menemui tipe data yang recursive
> Misal kita punya tipe data Category,
        dimana di dalamnya bisa terdapat Category lagi.
        Kita sering melihat jenis data seperti ini,
        contohnya di Toko Online
*/

#[derive(Debug)]
enum Category {
    Of(String, Box<Category>),
    End,
}

#[test]
fn test_box_enum() {
    let category = Category::Of(
        "laptop".to_string(),
        Box::new(Category::Of("DELL".to_string(), Box::new(Category::End))),
    );
    println!("{:?}", category)
}

// DEREFERENCE
/*
Saat kita menggunakan Reference, kadang kita ingin melakukan manipulasi data langsung ke Value nya
Kita bisa melakukan Dereference untuk mengakses langsung Value nya, ukan lagi Reference nya
Untuk melakukan Dereference, kita bisa menggunakan operator * (bintang)
*/

#[test]
fn test_dereference() {
    let val1 = Box::new(10);
    let val2 = Box::new(20);

    let res = *val1 * *val2;
    println!("result: {}", res)
}

/*
DEREF TRAIT
> Saat kita menggunakan Reference atau Box<T>, kita bisa menggunakan * Operator untuk melakukan Dereference
> Bagaimana jika kita menggunakan tipe lain? Misal Struct yang kita buat sendiri? Secara default kita tidak bisa menggunakan Deference
> Namun, jika kita ingin membuat Struct yang kita buat memiliki kemampuan Dereference, kita bisa menggunakan Deref Trait
> Khusus untuk Mutable Value, kita juga bisa menggunakan DerefMut
*/

struct MyValue<T> {
    value: T,
}

use std::ops::Deref;
impl<T> Deref for MyValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[test]
fn test_deref() {
    let value = MyValue { value: 100 };
    let real_value = *value;
    println!("{}", real_value);
}

/*
DEREF untuk Parameter
> Deref juga bisa digunakan untuk Parameter yang secara otomatis melakukan Reference ke Value yang ditunjuk pada implementasi yang kita buat
> Misal sebelumnya kita membuat MyValue<String>, lalu misal kita ingin mengirim ke function dengan parameter &String
> Kita bisa langsung menggunakan &my_value
*/

fn say_hello(name: &String) {
    println!("Hello: {}", name)
}

#[test]
fn test_deref_coercion() {
    let name = MyValue {
        value: "Aninditya".to_string(),
    };
    say_hello(&name)
}

/*

**CLEANUP**
DROP TRAIT
> Saat kita membuat value, ketika value tersebut keluar dari scope, secara otomatis value akan di drop (hapus) oleh Rust
> Drop Trait merupakan Trait yang bisa kita implementasikan, untuk membuat kode yang akan dieksekusi sebelum value di drop
> Misal menutup koneksi, resource dan lain-lain
*/

struct Book {
    title: String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping book: {}", self.title)
    }
}

#[test]
fn test_drop_book() {
    let book = Book {
        title: "Rust Programming".to_string(),
    };
    println!("Book: {}", book.title)
}

/*
*** MULTIPLE OWNERSHIP ***
> Pada umumnya, value biasanya hanya dimiliki oleh satu variable
>       Namun,
>       mungkin akan ada kasus dimana satu value dimiliki oleh beberapa variable,
>       contoh misal pada struktur data Graph, dimana satu titik bisa berasal dari beberapa titik
> Seperti yang kita tahu, bahwa defaultnya di Rust satu value hanya bisa dimiliki oleh satu variable
> Jika kita ingin membuat satu value bisa dimiliki oleh beberapa variable, kita harus menggunakan type Rc<T> (Reference Counted)
*/

// Rc<T>
// Rc<T> mirip seperti Box<T>
// Rc<T> atau Reference Counted adalah tipe data Smart Pointer yang bisa digunakan untuk lebih dari satu variable owner.

use std::rc::Rc;
enum Brand {
    Of(String, Rc<Brand>),
    End,
}

#[test]
fn test_multiple_ownership_box() {
    // let apple = Category::Of("Apple".to_string(), Box::new(Category::End));
    // let laptop = Category::Of("Laptop".to_string(), Box::new(apple));
    // let phone = Category::Of("Smartphone".to_string(), Box::new(apple));
    let apple = Rc::new(Brand::Of("apple".to_string(), Rc::new(Brand::End)));
    println!("Apple Reference count : {}", Rc::strong_count(&apple));
    let laptop = Brand::Of("laptop".to_string(), Rc::clone(&apple));
    println!("Apple Reference count : {}", Rc::strong_count(&apple));
    {
        let smart_phone = Brand::Of("smartphone".to_string(), Rc::clone(&apple));
        println!("Apple Reference count : {}", Rc::strong_count(&apple));
    }
    println!("Apple Reference count : {}", Rc::strong_count(&apple));
}

// STATIC ******************************************************************************
// sama seperti CONSTANT, menggunakan keyword "static"
static APPLICATION: &str = "My Application";

#[test]
fn test_statics() {
    println!("application: {}", APPLICATION)
}

//
// MUTABLE STATIC
/*
> Value di constant tidak bisa diubah lagi
    kita bisa buat mutable static, yang value nya bisa diubah lagi
> Namun karena static itu bisa diakses oleh siapapun, jadi ada kemungkinan tidak aman, misal terjadi RACE CONDITION
> Oleh karena itu untuk mengubah mutable static, kita wajib menggunakan "unsafe" block, atau "unsafe" function
*/
static mut COUNTER: u32 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

#[test]
fn test_mutable_static() {
    unsafe {
        increment();
        COUNTER += 1;
        println!("Counter: {}", COUNTER)
    }
}
