fn main() {
    println!("Hello, world!");
}

// CLOSURE atau ANONYMOUS FUNCTION
// adalah FUNCTION TANPA NAMA yang disimpan di Variable atau digunakan di Parameter
// untuk membuatnya, kita bisa menggunaan tipe data
//      fn(paramType) -> returnType
// dan untuk memanggil CLOSURE, kita bisa panggil menggunakan nama variable atau parameter -nya secara langsung

#[test]
fn test_closure() {
    let sum = |value1: i32, value2: i32| -> i32 { value1 + value2 };

    let res = sum(1, 2);
    println!("{}", res)
}

// CLOSURE as PARAMETER
fn print_with_filter(value: String, filter: fn(String) -> String) {
    let res = filter(value);
    println!("Result: {}", res);
}

#[test]
fn test_closure_as_parameter() {
    let name = String::from("Aninditya Qamela Sauqiya");
    print_with_filter(name, |value: String| -> String { value.to_uppercase() });

    // OR
    let name2 = String::from("Dedy Styawan");
    let filter = |value: String| -> String { value.to_lowercase() };
    print_with_filter(name2, filter)
}

// CLOSURE dari FUNCTION
// menggunakan CLOSURE dari FUNCTION yang sudah ada
//      hal ini bisa dilakukan cukup dengan menyebutkan nama_function sebagai CLOSURE

fn to_upper(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_closure_as_parameter2() {
    let name = String::from("Aninditya Qamela Sauqiya");
    print_with_filter(name, to_upper) // diapakai function nya , bukan di panggil, sehingga tanpa "()"
}

// CLOSURE SCOPE
#[test]
fn test_closure_scope() {
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Increment")
    };

    increment();
    increment();
    increment();
    println!("{}", counter)
}

// Better Way
struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
        println!("increment")
    }
}

#[test]
fn test_closure_scope_with_struct() {
    let mut counter = Counter { value: 0 };
    counter.increment();
    counter.increment();
    counter.increment();

    println!("{}", counter.value);
}
