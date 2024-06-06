// https://www.youtube.com/watch?v=FkASrE05VY4

fn main() {
    println!("Hello, world!");
}

/*
-----------------------------------------------------------------------------
SCALAR
-----------------------------------------------------------------------------
*/
#[test]
fn hello_test() {
    println!("{}", "hello test");
}

#[test]
fn test_variable() {
    let name = "Anin";
    println!("hello, {}", name)
}

#[test]
fn test_mutable() {
    #[allow(unused_assignments)]
    let mut name = "Anin";
    name = "Aninditya QS";
    println!("Hi, {}", name);
}

#[test]
fn test_shadowing() {
    let name = "Anin";
    println!("Heloo,{}", name);

    let name = 10;
    println!("count, {}", name);
}

#[test]
fn test_number() {
    let num1 = 10;
    println!("integer {}", num1);

    let num2: f32 = 10.5;
    println!("floating {}", num2);

    // conversion only from small to big data type (i32 -> i64), using as
    let num3: i32 = 10;
    let num4: i64 = num3 as i64;
    println!("conversion {}", num4);
}

#[test]
fn test_augmented_assignments() {
    let mut a = 10;
    println!("a: {}", a);
    a += 10;
    println!("a: {}", a);
    a -= 5;
    println!("a: {}", a);
    a *= 10;
    println!("a: {}", a);
    a /= 10;
    println!("a: {}", a);
    a %= 10;
    println!("a: {}", a);
}

#[test]
fn test_boolean() {
    let a = true;
    let b: bool = false;
    println!("{}, {}", a, b);

    let message_number = 3;
    let message3: i8 = 24;
    println!("message number {1} -> {0}", message3, message_number);
    println!("message number {}: {}", message_number, message3);
    println!("message number {0}: {1}", message_number, message3);
    println!("message number {1}: {0}", message3, message_number);
}

#[test]
fn test_char() {
    let char1 = 'a';
    let char2 = 'b';
    println!("{}, {}", char1, char2);
}

/*
-----------------------------------------------------------------------------
COMPOUND
-----------------------------------------------------------------------------
*/
// TUPLE ************************************************************************************************************************
#[test]
fn test_tuple() {
    // jumlah data di tuple sudah final, dan tidak bisa berkurang atau bertambah
    let data_tuple = (10, 5.5, true, "anin", 'a');
    println!("{:?}", data_tuple);

    // each individual
    println!("{}, {}", data_tuple.0, data_tuple.4);

    // destructuring, using "_" if not using
    let (a, b, _, d, e) = data_tuple;
    println!("{},{},{},{}", a, b, d, e);

    // default tuple -> IMMUTABLE
    // making tuple MUTABLE
    let mut data_tuple2 = (10, 5.5, true, "anin", 'a');
    data_tuple2.1 = 7.7;
    data_tuple2.2 = false;
    println!("{:?}", data_tuple2);
}

// UNIT ************************************************************************************************************************
// unit is an EMPTY TUPLE
#[allow(dead_code)]
fn unit() {
    println!("{}", "hello")
}

#[test]
fn test_unit() {
    let res = unit();
    println!("{:?}", res);

    let test = ();
    println!("{:?}", test);
}

// ARRAY ************************************************************************************************************************
#[test]
fn test_array() {
    // Remember, arrays are fixed-size and cannot grow or shrink dynamically.
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    let arr2: [i64; 3] = [10, 11, 12];
    println!("{:?}", arr2);

    let (a, b) = (arr[0], arr[4]);
    println!("{}, {}", a, b);

    // default Array -> IMMUTABLE
    // making Array MUTABLE
    let mut arr3 = [5.1, 6.2, 7.3];
    arr3[1] = 9.9;
    println!("{:?}", arr3);

    // usize (u32 or u64, depend on OS)
    println!("{}", arr3.len());
}

// TWO dimensional array
#[test]
fn test_to_dimensional_array() {
    let matrix: [[i32; 2]; 2] = [[1, 2], [3, 4]];
    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{:?}", matrix[1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
}

// CONSTANT ************************************************************************************************************************
#[allow(dead_code)]
const MINIMUM_VALUE: f64 = 90.7;
#[test]
fn test_constant() {
    // IMMUTABLE, build declaration, EXPLICIT data type, UPPER CASE
    const MAXIMUM_VALUE: i32 = 100;
    println!("{}, {}", MAXIMUM_VALUE, MINIMUM_VALUE);
}

// VARIABLE SCOPE ************************************************************************************************************************
// variables can be used only in the scope where the variable is defined, and also in the inner scope
// variable cannot use in outer scope
#[test]
fn test_scope_variable() {
    let a = 1; // variable scope

    {
        // inner scope
        println!("{}", a);
        let b = 2.1;
        println!("{}", b);
    }
    // println!("{}", b); // this will be error
}

/****** GARBAGE COLLECTION ******/
// Rust not using GC, devided data in memory on STACK and HEAP
// STACK -> adalah bagian dimana data disimpan dalam strucktur tumpukan, last in first out,
//          semua data di STACK harus fix size (ukuran data sudah pasti)
// HEAP  -> adalah tempat menyimpan data
//          di HEAP terdapat Memory Allocator (bertugas menemukan area kosong untuk menyimpan & alokasi data ke area tersebut)
//          setelah itu kita akan di beri Pointer (petunjuk/address) ke lokasi dimana data itu berada di HEAP
//          Pointer dari HEAP berukuran fix size, sehingga Pointer tersebut akan di simpan di STACK

//    STACK                             HEAP
//
// │    ↓     │           │  STACK   │         │    HEAP    │
// │ 0x000001 │           │          │         │  ┌──────┐  │
// │ 0x000002 │           │          │    ┌──→ │  │ data │  │
// │ 0x000003 │           │ 0x000005 │ ───┘    │  └──────┘  │
// │ 0x000004 │           │ <addres> │         │            │
//      ↓
//   Last In
//  First Out

#[allow(dead_code)]
fn function_a() {
    let a = 10; // stack
    let b = String::from("this will save on HEAP");
    println!("{}\n {}\n ", a, b);
}
#[test]
fn test_stack_heap() {
    function_a();
}

//
// │         STACK         │                   . -- ~~~ -- .
// │                       │               .-~     HEAP       ~-.
// │  ┌─────────────────┐  │              /                      \
// │  │  function_a()   │  │             /                        \
// │  │                 │  │            |                          |
// │  │ ┌─────┐ ┌─────┐ │  │     ┌──────┼→"this will save on HEAP" |
// │  │ │  a  │ │  b  ├─┼──┼─────┘      |                          |
// │  │ └─────┘ └─────┘ │  │             \                        /
// │  └─────────────────┘  │              \                      /
// │                       │               `-.                .-'
//                                             ~- . ____ .  -~

// STRING
// perbedaan:
// &str (string slice)  -> fix size, save in stack. immutable
// String -> not fix size, save in HEAP
//           ketika dalam bentuk IMMUTABLE variable, maka String tidak bisa berkembang, tapi tetap disimpan di HEAP
//           ketika dalam bentuk MUTABLE variable, maka String bisa berkembang di dalam HEAP
//           String punya method untuk manipulasi data, perlu di perhatikan ada method untuk mengubah datanya sendiri,
//              namun ada method yang mengubah datanya dalam bentuk data baru, tanpa memodifikasi data aslinya
//              https://docs.rust-lang.otg/srd/string/struct.String.html

#[test]
fn test_string_slice() {
    let name = "        Aninditya Qamela Sauqiya ";
    let trim = name.trim(); // this will copy
    println!("lenght    : {}", name.len());
    println!("address   : {:p}", name); // difference address
    println!("value     : {}", name);
    println!("lenght    : {}", trim.len());
    println!("address   : {:p}", trim); // difference address
    println!("value     : {}", trim);
}

#[test]
fn test_string_type() {
    let mut name = String::from("Aninditya");
    name.push_str(" Qamela S");
    println!("{}", name);

    // harus disimpan di variable baru untuk menampilkan perubahan
    let full_name = name.replace("S", "Sauqiya");
    println!("{}", full_name);
}

// OWNERSHIP ************************************************************************************************************************
// rules of the game:
// 1. Setiap value di Rust harus punya OWNER (variable pemilik value)
// 2. Dalam satu waktu, hanya boleh 1 OWNER
// 3. Ketika OWNER keluar Scope, value akan di hapus

#[test]
fn test_ownership_rules() {
    // a tidak bisa di akses disini, belum di deklarasikan
    let a = 10; // a bisa di akses mulai dari sini
    {
        // b tidak bisa di akses disini, belum di deklarasikan
        let b = 20; // b bisa di akses mulai dari sini
        println!("{}", b);
    } // scope b selesai, b di hapus, b tidak bisa di akses lagi

    println!("{}", a);
} // scope a selesai, a di hapus, a tidak bisa di akses lagi

// OWNERSHIP MOVEMENT (Only on HEAP)
// rules :
// 1. Data Copy tidak terjadi untuk tipe data yang di simpan di HEAP
// 2. Dalam satu waktu, hanya boleh 1 OWNER
// 3. Maka, ketika kita buat variable Baru (Owner Baru) dari variable lama (Owner Lama),
//      maka yg terjadi bukanlah Copy,
//      melainkan transfer ownership, dari Owner Lama ke Owner Baru
// 4. Setelah proses transfer Ownership selesai, otomatis Owner Lama tidak Valid digunakan

#[test]
fn test_ownership_movement() {
    let name1 = String::from("Anin");

    // ownership dari name1 dipindahkan ke name2
    let name2 = name1;
    // name1 tidak bisa diakases dari sini

    // println!("{}", name1); // error
    println!("{}", name2);
}

// CLONE (Only on HEAP)
// perbedaan Copy & Clone:
// Copy  -> di Stack, tipe data yang fix, data awal tidak di hapus
// Clone -> di HEAP, tipe data yg tidak fix, data awal di pindahkan ownershipnya
//
// rules:
// 1. Clone membuat data tiruan yang sama dari data aslinya
// 2. String, memiliki method clone()
// 3. saat memanggil method clone(), maka method tersebut akan meng-copy data String menjadi data String baru
// 4. semua type data yang di simpan di HEAP, memiliki method clone()

#[test]
fn test_clone() {
    let name1 = String::from("dedy");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

//
// │      STACK                          . -- ~~~ -- .
// │                                  .-~     HEAP     ~-.
// │  ┌─────────────┐                /                    \
// │  │test_clone() │               /                      \
// │  │             │              |                        |
// │  │  ┌───────┐  │       ┌──────┼───→"dedy"              |
// │  │  │ name1 ├──┼───────┘      |                        |
// │  │  └───────┘  │               \                      /
// │  │  ┌───────┐  │       ┌────────\──→"dedy"            /
// │  │  │ name2 ├──┼───────┘         `-.              .-'
// │  │  └───────┘  │                     ~-. _____ .-~
// │  └─────────────┘
//

// FUNCTION ************************************************************************************************************************
#[allow(dead_code)]
fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    return result;
}

#[test]
fn test_factorial_loop() {
    println!("{}", factorial_loop(5));
    println!("{}", factorial_loop(-10));
}

// RECURSIVE FUNCTION
#[allow(dead_code)]
fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Dedy"), 5);
}

// --------
#[allow(dead_code)]
fn factorial_recursive(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    // let x = factorial_recursive(n - 1);
    // println!("{} {}", n, x);
    // n * x
    println!("{}", n);
    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("result: {}", result);
}

// Ownership and Function
// Type Data -> yang di simpan di HEAP -> dan -> dikirim sebagai Parameter -> Function,
//      maka secara otomatis Ownership -> pindah -> ke parameter, di Function yang di Panggil
//      Karena Ownership pindah ke Parameter Function
// Namun...
// Apabila Type Data di Stack -> dikirim -> Parameter -> Function, Maka Value akan di Copy

#[allow(dead_code)]
fn print_number(number: i32) {
    println!("number: {}", number);
}

#[allow(dead_code)]
fn hi(name: String) {
    println!("Hi, {}", name)
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Dedy");
    hi(name);
    // println!("{}", name); // error cause variable "name" Ownership move to -> Parameter Function "hi(name: String)"
}

// REFERENCES ************************************************************************************************************************
#[allow(dead_code)]
fn hi2(name: &String) {
    println!("Hi, {}", name)
}

#[test]
fn test_reference() {
    let name = String::from("Dedy");
    hi2(&name);
    println!("{}", name);
}

// BORROWING / REFERENCES ************************************************************************************************************************
// Saat kita mencoba memodifikasi Value dari REFERENCES, secara default tidak bisa, (REFERENCE adalah IMMUTABLE)
//      walaupuyn Variable OWNER-nya adalah MUTABLE
// this example produce error

#[allow(dead_code, unused_variables)]
fn change_value(value: &String) {
    // value.push_str("Test"); // cannot borrow `*value` as mutable, as it is behind a `&` reference
}

// MUTABLE REFERENCE (&mut)
// Dalam satu waktu, hanya boleh membuat 1 mutable REFERENCE dan tidak ada REFERENCE lainnya
#[allow(dead_code)]
fn change_value2(value: &mut String) {
    value.push_str(" Adding")
}

#[test]
fn test_mutable_reference() {
    let mut value = String::from("Original");
    // ini diperbolehkan, karena life cycle variable mutable reference dari "value", di function change_value2()
    // sehingga value langsung di hapus saat keluar dari function change_value2()
    change_value2(&mut value);
    change_value2(&mut value);
    change_value2(&mut value);
    println!("{}", value);
    // or
    let value_borrow = &mut value;
    change_value2(value_borrow);
    println!("{}", value);

    /*
        // ini tidak diperbolehkan, karena life cycle variable mutable reference dari "value" dalam 1 scope test_mutable_reference()
        let val_borrow1 = &mut value;
        let val_borrow2 = &mut value;
        let val_borrow3 = &mut value;
        // Or
        let val_borrow1 = &mut value;
        let val_borrow2 = &value;
    */
}

// DANGLING POINTER (Not Allow)
// adalah REFERENCE yang menunjuk ke value yang tidak ada di MEMORY
/*
fn get_full_name(first_name: &String, last_name: &String) -> &String {  // missing lifetime specifier
    let name = format!("{} {}", first_name, last_name);
    return &name; // this will cause error
}
*/

// STRUCT ************************************************************************************************************************
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

#[allow(dead_code)]
fn print_person(person: &Person) {
    //using &Person (REFERENCE of PERSON), agar tidak tidak terjadi transfer ownership
    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

// instance of struct
#[test]
fn test_finstance_struct() {
    let person = Person {
        first_name: String::from("Aninditya"),
        last_name: String::from("QS"),
        age: 10,
    };
    print_person(&person); // dikirim sebagai REFERENCE
}

// Init Shorthand
#[test]
fn test_init_shorthand() {
    let first_name = String::from("Aninditya");
    let last_name = String::from("QS");

    let person = Person {
        first_name,
        last_name,
        age: 10,
    };

    // Perlu di ingat bahwa dengan init shorthand menggunakan variable,
    // maka OWNERSHIP nya dari first_name & last_name akan berpindah ke STRUCT Person,
    // mengapa berpindah karena type STRING, yang ada di HEAP
    // sehingga println! di bawah ini akan menjadi ERROR
    // println!("{}", first_name);

    print_person(&person)
}

// UPDATE SYNTAX
// Becareful using struct update syntax, jika ada field yang value-nya di HEAP, karena OWNERSHIP-nya akan otomatis pindah ke field di INSTANCE BARU
#[test]
fn test_mutable_struct() {
    let first_name = String::from("Aninditya");
    let last_name = String::from("QS");

    let person = Person {
        first_name,
        last_name,
        age: 10,
    };

    let person2 = Person { ..person };
    print_person(&person2);
}

// PARTIAL STRUCT UPDATE SYNTAX
#[test]
fn test_clone_struct() {
    let first_name = String::from("Aninditya");
    let last_name = String::from("QS");

    let person = Person {
        first_name,
        last_name,
        age: 10,
    };

    let person2 = Person {
        first_name: person.first_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };
    print_person(&person);
    print_person(&person2);

    println!("{}", person.first_name);
    println!("{}", person.last_name);
}

// TUPLE STRUCT
#[allow(dead_code)]
struct GeoPoint(f64, f64);

#[test]
fn test_tuple_struct() {
    let geo_point = GeoPoint(-6.20000, 106.81666);
    println!("long : {}", geo_point.0);
    println!("lat  : {}", geo_point.1);
}

// STRUCT TANPA FIELD
// Struct tanpa FIELD sama saja dengan tipe data Unit()
// Next penggunaannya untuk Trait

#[allow(dead_code)]
struct Nothing;
#[test]
fn test_nothing() {
    let _nothing = Nothing;
    let _nothing2 = Nothing {};
}

// REFERENCE FIELD on STRUCT
// menggunakan LIFETIME

// METHOD ************************************************************************************************************************
// Sama seperti FUNCTION
// Namun METHOD tidak berdiri sendiri, melainkan menempel di STRUCt, ENUM, atau TRAIT
// Pada METHOD, Parameter pertama selalu menggunakan "self"
// "Self" adalah representasi dari INSTANCE dari STRUCT, dimana METHOD tersebut di panggil
// HOW TO BUILD
// untuk membuat method, tentukan ingin meletakkan di STRUCT mana, caranya menggunakan keyword "impl", lalu di ikuti nama STRUCT -nya
// lalu di dalamnya kita bisa lakukan seperti membuat FUNCTION
// untuk mengakses semua Field yang ada di instance STRUCT, kita bisa gunakan parameer SELF pertama di METHOD
// biasanya param "self" dibuat dalam bentuk REFERENCE, agar OWNERSHIP nya tidak diambil oleh METHOD yang dipanggil

impl Person {
    #[allow(dead_code)]
    fn say_hello(&self, name: &str) {
        println!("Hello, {} my name is {}", name, self.first_name);
    }
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Aninditya"),
        last_name: String::from("QS"),
        age: 10,
    };
    // calling method
    person.say_hello("Dedy");
}

// ASSOCIATED FUNCTIONS
// setiap FUNCTION yang kita buat di dalam "impl" kita sebut ASSOCIATED FUNCTIONS, karena terkait dengan tipe data yang di tentukan di "impl"
// ASSOCIATED FUNCTIONS yang memiliki parameter "self" adalah METHOD, dan di panggil setelah membuat INSTANCE -nya
// Namun...kita bisa membuat FUNCTION tanpa paramter "self", yang artinya FUNCTION tersebut tidak terhubung dengan INSTANCE -nya
// untuk memanggil ASSOCIATED FUNCTIONS yang bukan METHOD, langsung gunakan NamaType::nama_function()
// biasanya ASSOCIATED FUNCTIONS bukan METHOD, digunakan untuk membuat INSTANCE dari TYPE-nya

impl GeoPoint {
    // berdiri sendiri
    #[allow(dead_code)]
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(-6.233233, 106.816666);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

// ENUM ************************************************************************************************************************
#[allow(dead_code)]
enum Level {
    REGULAR,
    PREMIUM,
    PLATINUM,
}

#[test]
fn test_enum() {
    let _level = Level::PLATINUM;
}

// ENUM DATA
#[allow(dead_code)]
enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWAllet(String, String),
}

#[test]
fn test_enum_data() {
    let _payment = Payment::BankTransfer(String::from("BCA"), String::from("1212312312"));
}

// ENUM METHOD
// sama seperti menambah METHOD di STRUCT
// impl Payment {
//     fn pay(&self, amount: u32) {
//         println!("Paying Amount: {}", amount)
//     }
// }

#[test]
fn test_payment() {
    let payment = Payment::BankTransfer(String::from("BCA"), String::from("1212312312"));
    payment.pay(10000);
}

// PATTERN MATCHING ************************************************************************************************************************
// For ENUM
#[test]
fn test_pm_enum() {
    let level = Level::PLATINUM;
    match level {
        Level::REGULAR => {
            println!("{}", "REGULAR");
        }
        Level::PREMIUM => {
            println!("{}", "PREMIUM");
        }
        Level::PLATINUM => {
            println!("{}", "PLATINUM");
        }
    }
}

// DESTRUCTURING ENUM DATA PATTERNS
impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!(
                    "Paying with bank transfer {} {} amount {}",
                    bank, number, amount
                );
            }
            Payment::EWAllet(wallet, number) => {
                println!(
                    "Paying with e-Wallet {} {} amount {}",
                    wallet, number, amount
                )
            }
        }
    }
}

#[test]
fn test_enum_destructuring() {
    let payment1 = Payment::CreditCard(String::from("323234323223"));
    payment1.pay(1000);
    let payment2 = Payment::BankTransfer(String::from("BCA"), String::from("1212312312"));
    payment2.pay(10000);
    let payment3 = Payment::EWAllet(String::from("OVO"), String::from("1218812"));
    payment3.pay(1000);
}

// PATERN MATCHING VALUE
#[test]
fn test_match_value() {
    let name = "Joko";
    match name {
        "Dedy" => {
            println!("{}", "Hello, Dedy")
        }
        "Joko" => {
            println!("{}", "Hello, Joko")
        }
        _other => {
            println!("{}", "Siapa kamu ?")
        }
    }
    // Multiple Matching
    match name {
        "Dedy" | "Joko" => {
            println!("{}", "Hello, Boss")
        }
        _other => {
            println!("{}", "Siapa kamu ?")
        }
    }
}

// Range Matching
#[test]
fn test_range_matching() {
    let value = 10;
    match value {
        0..=3 => {
            println!("{}", "bad")
        }
        4..=6 => {
            println!("{}", "not bad")
        }
        7..=9 => {
            println!("{}", "Good")
        }
        10 => {
            println!("{}", "Great")
        }
        _other => {
            println!("{}", "unknown")
        }
    }
}

// DESTRUCTURING STRUCT PATTERNS
#[test]
fn test_destructuring_tuple() {
    let point = GeoPoint(0.0, 11.0);
    match point {
        GeoPoint(long, 0.0) => {
            println!("long: {}", long)
        }
        GeoPoint(0.0, lat) => {
            println!("lat: {}", lat)
        }
        GeoPoint(long, lat) => {
            println!("long: {}, lat: {}", long, lat)
        }
    }
}

#[test]
fn test_destruct_struct_pattern() {
    let person = Person {
        first_name: String::from("Dedy"),
        last_name: String::from("Styawan"),
        age: 30,
    };
    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("First Name: {}, Last Name: {}", first_name, last_name)
        }
    }
}

// IGNORING
// Sebelumnya di Struct jika kita tidak butuh fieldnya, kita bisa gunakan .. (dua titik)
// Namun pada kasus Tuple STRUCT,ENUM, kita tidak bisa !, karena posisi field sudah diatur sesuai posisinya
//      Jika kita tidak butuh filed tersebut, bisa diganti menjadi _ (underscore)
//      Jika kita tidak butuh data apapun, bisa gunakan _ (underscore)

#[test]
fn test_ignoring_tuple() {
    let point = GeoPoint(0.0, 11.0);
    match point {
        GeoPoint(long, _) => {
            println!("long: {}", long)
        }
    }
}

#[test]
fn test_ignoring_range_matching() {
    let value = 10;
    match value {
        0..=3 => {
            println!("{}", "bad")
        }
        4..=6 => {
            println!("{}", "not bad")
        }
        7..=9 => {
            println!("{}", "Good")
        }
        10 => {
            println!("{}", "Great")
        }
        _ => {
            println!("{}", "unknown")
        }
    }
}

// TYPE ALIAS ************************************************************************************************************************

type Age = u8;
type KTP = String;

struct Customer {
    id: KTP,
    name: String,
    age: Age,
}

#[test]
fn test_type_alias() {
    let cust = Customer {
        id: String::from("11111"),
        name: String::from("Anin"),
        age: 10,
    };

    println!("{}, {}, {}", cust.id, cust.name, cust.age)
}

// MODULE ************************************************************************************************************************
