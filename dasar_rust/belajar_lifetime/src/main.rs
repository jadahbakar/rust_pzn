fn main() {
    println!("Hello, world!");
}

// Di materi ownership dan reference,
//      kita sudah tau bahwa tiap data / reference memiliki lifetime (alur hidup) yang sudah ditentukan
// Secara default,
//      Lifetime di Rust sudah ditentukan mengikuti scope variable,
//      sehingga aman dan Rust juga melakukan borrow check pada saat melakukan kompilasi
//      untuk memastikan tidak ada masalah yang bernama Dangling Reference (reference ke value yang sudah tidak ada di memory)

// this is sample error ------------------------------------
// #[test]
// fn test_dangling_reference() {
//     let r: i32;
//     {
//         let x: i32 = 5;
//         r = &x; // error karena x sudah di hapus ketika keluar dari scope
//     }
//     println!("{}", r)
// }

// fn test_longest(val1: &str, val2: &str) -> &str { //missing lifetime specifier
//     if val1 > val2 {
//         val1
//     } else {
//         val2
//     }
// }
// ---------------------------------------------------------

// LIFETIME ANOTATION
// pada kasus Function longest, Rust menyediakan fitur -> Lifetime Annotation
// cara nya dengan menambahkan tanda petik satu (" ' "), didepan type aslinya

fn longest<'a>(val1: &'a str, val2: &'a str) -> &'a str {
    if val1 > val2 {
        val1
    } else {
        val2
    }
}

#[test]
fn test_litetime_annotation() {
    let val1 = "Aninditya";
    let val2 = "Qamela";
    let res = longest(val1, val2);
    println!("{}", res)
}

// LIFETIME ANOTATION Tidak mengubah waktu hidup
//      Hanya penanda untuk membantu Rust BORROW CHECKER
//      oleh karena itu, jika alur hidup Variable sudah selesai, maka bisa saja terjadi Error (Dangling Reference)
// contoh: error

// #[test]
// fn test_lifetime_annotation_dangling_reference() {
//     let string1 = String::from("string 1");
//     let result;
//     {
//         //inner scope
//         let string2 = String::from("string 2");
//         result = longest(string1.as_str(), string2.as_str())
//         /* this will produce error:
//         `string2` does not live long enough. borrowed value does not live long enough
//         */
//     }
//     println!("{}", result);
// }

// LIFETIME ANOTATION di STRUCT
// mirip seperti GENERIC dan bisa digunakan di STRUCT
// dengan menggunakan LIFETIME ANOTATION di STRUCT,
//      kita bisa menandai Field dengan tipe Reference
// sehingga nanti, kita bisa menggunakan LIFETIME ANOTATION di STRUCT tersebut

struct Student<'a> {
    name: &'a str,
}
fn longest_student_name<'a>(student1: &Student<'a>, student2: &Student<'a>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
    }
}

#[test]
fn test_lta_on_struct() {
    let std1 = Student { name: "Aninditya" };
    let std2 = Student { name: "Qamela" };

    let res = longest_student_name(&std1, &std2);
    println!("Student : {}", res)
}

// LIFETIME ANOTATION di METHOD
impl<'a> Student<'a> {
    fn longest_name(&self, student: &Student<'a>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

#[test]
fn test_lta_on_method() {
    let student1 = Student { name: "Aninditya" };
    let student2 = Student { name: "Qamela" };

    let res = student1.longest_name(&student2);
    println!("Student : {}", res)
}

// LIFETIME ANOTATION dan GENERIC TYPE
// bisa di gabungkan bersama Generic Type
// contoh:

struct Teacher<'a, ID>
where
    ID: Ord,
{
    id: ID,
    name: &'a str,
}

#[test]
fn test_lta_generic_struct() {
    let teacher = Teacher {
        id: 1,
        name: "Anin",
    };
    println!("teacher: {}, {}", teacher.id, teacher.name)
}
