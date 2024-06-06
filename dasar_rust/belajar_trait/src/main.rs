fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
struct Person {
    first_name: String,
    last_name: String,
    id: u8,
}

// TRAIT == Interface == Contract
// implementasi trait bisa menggunakan Struct atau ENUM
// impl <NamaTrait> for <NamaType> {
//      ...isi_method...
// }

#[allow(dead_code)]
trait CanSayHello {
    fn hello(&self) -> String {
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
    fn say_full_name(&self) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("hello, my name is {}", self.first_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("hello {}, my name is {}", self.first_name, name)
    }
    fn say_full_name(&self) -> String {
        format!("Hi, my name is {} {}", self.first_name, self.last_name)
    }
}

// Trait tidak bisa dibuat instance -nya
// untuk membuat instance dengan tipe data Trait, maka kita harus gunakan implementasi (di STRUCT / ENUM) -nya

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Aninditya"),
        last_name: String::from("QS"),
        id: 10,
    };

    println!("{}", person.say_hello());
    println!("{}", person.say_hello_to("Dedy"));
    println!("{}", person.say_full_name());
    // this is default implementation
    println!("{}", person.say_hello());
}

// Pros using TRAIT
// TRAIT sebagai parameter
//  Saat TRAIT menjadi Parameter, maka kita bisa gunakan value apapun yang merupakan implementasi dari TRAIT, sebagai value untuk Parameter
//      kita bisa gunakan kata kuncu "impl <NamaTrait>" pada parameternya
//          atau menggunakan REFERENCE -> "&impl <NamaTrait>"
#[allow(dead_code)]
fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_full_name());
}

#[test]
fn test_parameter() {
    let person = Person {
        first_name: String::from("Aninditya"),
        last_name: String::from("QS"),
        id: 10,
    };
    say_hello_trait(&person);
}

// Multiple TRAIT
// Type bisa mengimplementasikan lebih dari satu TRAIT
//  oleh karena itu, saat membuat Parameter, kita bisa buat satu Parameter utnuk beberapa tipe TRAIT
//  bisa juga dengan tanda "+" dengan tipe Multiple TRAIT, (impl Trait1 + Trait2 + Trait3)

#[allow(dead_code)]
trait SayGoodBye {
    fn good_bye(&self) -> String;
}

impl SayGoodBye for Person {
    fn good_bye(&self) -> String {
        format!("Hi, I'm {}, Bye", self.first_name)
    }
}
#[allow(dead_code)]
fn multiple_trait(value: &(impl CanSayHello + SayGoodBye)) {
    println!("{}", value.say_hello());
    println!("{}", value.good_bye());
}

#[test]
fn test_mult_trait() {
    let person = Person {
        first_name: String::from("Aninditya"),
        last_name: String::from("QS"),
        id: 10,
    };

    multiple_trait(&person);
}

// RETURN TRAIT
// TRAIT bisa digunakan sebagai RETURN VALUE di Function
// karena trait tidak bisa dibuat INSTANCE -nya secara langsung
//      maka VALUE yang dikembalikan juga harus dalam bentuk IMPLEMENTASI TYPE -nya
// untuk membuatnya sebagai RETURN VALUE -> impl <NamaTrait>

#[allow(dead_code)]
struct SimplePerson {
    name: String,
}

#[allow(dead_code)]
trait WhoAmI {
    fn introduce(&self) -> String;
}

impl WhoAmI for SimplePerson {
    fn introduce(&self) -> String {
        format!("Hi, My name is {}", self.name)
    }
}

#[allow(dead_code)]
fn return_value(value: String) -> impl WhoAmI {
    SimplePerson { name: value }
}

#[test]
fn test_return_value() {
    let person = return_value(String::from("Anin"));
    println!("{}", person.introduce());
}

// SUPER TRAIT
// pewarisan / inheritance

#[allow(dead_code)]
trait CanSay: CanSayHello + SayGoodBye {
    // sekarang jika kita implementasi Cansay,
    // kita harus implementasi CanSayHello & SayGoodBye
    fn say(&self) {
        println!("{}", self.say_hello());
        println!("{}", self.good_bye());
    }
}

impl CanSay for Person {}

#[test]
fn test_super_trait() {
    let person = Person {
        first_name: String::from("Aninditya"),
        last_name: String::from("QS"),
        id: 11,
    };
    println!("{}", person.hello());
    println!("{}", person.good_bye())
}
