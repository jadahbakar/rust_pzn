use std::collections::{LinkedList, VecDeque};

fn main() {
    println!("Hello, collection!");
}

// COLLECTION ********************************************
// sebelumnya kita sudah membahas ARRAY, namun ARRAY tidak bisa berkembang ukurannya (disimpan di STACK)
//  COLLECTION bisa berkembang, dan di simpan di HEAP.
//      nb: STRING merupakan tipe data COLLECTION, COLLECTION of Char
// COLLECTION ada 3: SEQUENCE, MAPS, SETS
// SEQUENCE -> tipe data yang memiliki index
// MAPS -> tipe data yang berupa key, value
// SETS -> tipe data yang tidak memiliki index

// SEQUENCE ************************************************************************************************************************************
// terdiri dari Vec (Vector), VecDeque, LinkedList
//
//
// Vec
//  sesuai urutan, nenambah di bagian belakang (Last In First Out)

#[test]
fn test_vector() {
    // kenapa names menggunakan mut, karena akan di masukkan data "push"
    let mut names = Vec::<String>::new();
    names.push(String::from("Aninditya"));
    names.push(String::from("Qamela"));
    names.push(String::from("Sauqiya"));

    for name in names {
        println!("{}", name)
    }

    // println!("{:?}", names) // error: borrow of moved value: `names`
    // hati-hati saat menggunakan Vec, karena di simpan di HEAP,
    // sehingga saat di gunakan panggil OWNERSHIP nya di kirim,
    //      akibatnya kode diatas "println!" akan error karena OWNERSHIP dari "names" sudah di gunakan di "for"
    // berbeda dengan ARRAY yang tipe data di simpan di STACK
    // contoh

    let arr = ["Aninditya", "Qamela", "Sauqiya"];
    for name in arr {
        println!("{}", name);
    }

    println!("{:?}", arr);

    // alternatif nya menggunakan REFERENCES atau POINTER
    let mut names = Vec::<String>::new();
    names.push(String::from("Aninditya"));
    names.push(String::from("Qamela"));
    names.push(String::from("Sauqiya"));

    for name in &names {
        println!("{}", name)
    }
    println!("{:?}", names);
    println!("{}", names[0]); // bisa menggunakan INDEX
}

// VecDeque
// Mirip Vec,
// yang membedakan, kemampuan untuk menambah di depan (head) dan di belakang (end)
//  implementasi QUEUE (First In First Out)

#[test]
fn test_vecdeque() {
    let mut data = VecDeque::new();
    data.push_back("satu");
    data.push_back("dua");
    data.push_front("tiga");

    for n in &data {
        println!("{}", n)
    }
    println!("{}", data[0]); // bisa menggunakan INDEX
}

// LinkedList
// Implementasi SEQUENCE menggunakan struktur data Linked List
// efisien untuk penambahan dan pengurangan data
// tidak terprediksi ukurannya
// downside nya performa nya tidak sebaik Vec
//      tidak punya fitur untuk mengakses data menggunakan index
#[test]
fn test_linked_list() {
    let mut data = LinkedList::new();
    data.push_back("satu");
    data.push_back("dua");
    data.push_front("tiga");

    for n in &data {
        println!("{}", n)
    }
    // println!("{}", data[0]); // tidak bisa menggunakan INDEX
}
