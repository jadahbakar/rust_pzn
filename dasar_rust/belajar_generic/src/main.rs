fn main() {
    println!("Hello, world!");
}

// GENERIC
// merupakan fitur dimana kita bisa membuat FUNCTION, STRUCT, ENUM, METHOD, TRAIT, yang Tipe datanya bisa di ubah ketika digunakan
// berguna jia membuat code yang GENERIC / GENERAL utnuk berbagai tipe data
//      sehingga kita tidak perlu tentukan dari awal tipe data -nya

// GENERIC on SRUCT
// NamaStruct <tipe_generic_1, tipe_generic_2>
// Biasanya hanya menggunakan satu huruf kapital

struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> { x: 5, y: 10 };
    let float = Point::<f32> { x: 7.3, y: 5.9 };

    println!("{} {}", integer.x, integer.y);
    println!("{} {}", float.x, float.y)
}

// GENERIC on ENUM
// same as STRUCT

#[allow(dead_code)]
enum VALUE<T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_generic_enum() {
    let value = VALUE::<i32>::VALUE(10);
    match value {
        VALUE::NONE => {
            println!("none")
        }
        VALUE::VALUE(value) => {
            println!("value :{}", value)
        }
    }
}

// GENERIC Type BOUND
// Memberikan batasan type yang diperbolehkan
// caranya menggunakan ":" (titik dua) diikuti dengan TRAIT
//      artinya -> GENERIC TYPE yang diperbolehkan hanya Implementasi dari TRAIT tersebut
// jika ingin menggunakan MULTIPLE TRAIT, bisa menggunakan "+"

struct SimplePerson {
    name: String,
}

trait SayGoodBye {
    fn good_bye(&self) -> String;
}

impl SayGoodBye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Hi, I'm {}, Bye", self.name)
    }
}

struct Hi<T: SayGoodBye> {
    value: T,
}

#[test]
fn test_generic_struct_with_trait() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson {
            name: String::from("Aninditya"),
        },
    };
}

// GENERIC on FUNCTION
// tambahkan nama Generic di function

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn test_generic_on_fuction() {
    let result = min::<i32>(10, 30);
    println!("{}", result);
    // rust sudah smart jadi bisa dengan seperti ini, karena kedua value "i32", dan return "i32"
    let res = min(13, 15);
    println!("{}", res);
}

// GENERIC on METHOD
// tambahkan tipe generic setelah kata kunci "impl"
// atau jika hanya untuk method tertentu bisa tambahkan generic type seperti di function

struct TwoDimensional<T> {
    x: T,
    y: T,
}

impl<T> TwoDimensional<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y<A>(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_method() {
    let point = TwoDimensional { x: 5, y: 8 };
    println!("{}", point.get_x());
    println!("{}", point.get_y::<i32>());
}

// GENERIC on TRAIT
// Saat membuat Generic Type di TRAIT -> otomatis memaksa -> Implementasi Trait, menggunakan Generic Type di Implementasi.

trait GetValue<T> {
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for TwoDimensional<T> {
    fn get_value(&self) -> &T {
        &self.x
    }
}

fn test_generic_on_trait() {
    let kotak = TwoDimensional { x: 11, y: 20 };
    println!("lebar kotak : {}", kotak.get_value())
}

// WHERE CLAUSE
// sebelumnya di BOUND kita menggunakan ":" diikuti nama TRAIT
//  kita bisa menggunakan WHERE
//  ini akan lebih mudah di baca ketika TYPE BOUND sangat banyak

trait GetShape<T>
where
    T: PartialOrd,
{
    fn get_panjang(&self) -> &T;
}

impl<T> GetShape<T> for TwoDimensional<T>
where
    T: PartialOrd,
{
    fn get_panjang(&self) -> &T {
        &self.x
    }
}

#[test]
fn test_generic_on_shape() {
    let kotak = TwoDimensional { x: 10, y: 10 };
    println!("panjang: {}", kotak.get_panjang())
}

// DEFAUT GENERIC TYPE
// menambahkan default type menggunakan tanda "="
//      artinya saat kita tidak menentukan tipe generic nya, secara otomatis akan menggunakan tipe data tersebut.

struct Lokasi<T = f32> {
    long: T,
    lat: T,
}

#[test]
fn test_genenric_default_value() {
    let pin = Lokasi {
        long: 100.2887,
        lat: 200.909,
    };
    println!("longitude: {}, latitude: {}", pin.long, pin.lat);

    let pin2 = Lokasi::<i32> { long: 10, lat: 20 };
    println!("longitude: {}, latitude: {}", pin2.long, pin2.lat);

    let pin3 = Lokasi::<String> {
        long: String::from("20 derajat"),
        lat: String::from("15 derajat"),
    };
    println!("longitude: {}, latitude: {}", pin3.long, pin3.lat);
}

// OVERLOADABLE OPERATOR *******************************************************
#[derive(Debug)]
struct Apple {
    quantity: i32,
}
use core::ops::Add;

// trait sudah ada di core::ops (bawaan rust)
impl Add for Apple {
    type Output = Apple;

    // rhs: Self, artinya memanggil dia sendiri == "impl Add<Apple>"
    //
    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_override() {
    let buah1 = Apple { quantity: 10 };
    let buah2 = Apple { quantity: 10 };

    let keranjang = buah1 + buah2;

    println!("jumlah buah: {}", keranjang.quantity);
}

// OPTIONAL VALUES *******************************************************
// use core::option;

fn double(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * 3),
    }
}

#[test]
fn test_optional_value() {
    let res = double(Some(3));
    println!("{:?}", res);

    let res = double(None);
    println!("{:?}", res);
}

// noted: can also using -> double(Option::Some(3)) or double(Option::None);

// COMPARING *******************************************************
use core::cmp::{Ordering, PartialOrd};

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //original -> Option<std::cmp::Ordering>
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_comparing() {
    let buah1 = Apple { quantity: 100 };
    let buah2 = Apple { quantity: 100 };

    println!("{}", buah1 == buah2);
    println!("{}", buah1 > buah2);
    println!("{}", buah1 < buah2);
}

// STRING MANIPULATION *******************************************************
#[test]
fn test_string_manipulation() {
    let s = String::from("Aninditya Qamela Sauqiya");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.contains("Q"));
    println!("{:?}", s.get(0..4));
    println!("{}", s.ends_with("Sauqiya"));
}

// FORMATING *******************************************************

struct Category {
    id: String,
    name: String,
}

use std::fmt::{Debug, Formatter};

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_formating() {
    let person = Category {
        id: String::from("Nama"),
        name: String::from("Aninditya"),
    };
    println!("{:?}", person);
}
