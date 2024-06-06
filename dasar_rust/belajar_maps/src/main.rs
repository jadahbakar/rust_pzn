use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

// MAP ***********************************************************************************************
// Tipe Data Collection yang berisi KEY - VALUE
// karena KEY itu unique,
//      jika kita memasukkan key value yg sama, maka akan me-REPLACE yang EXISTING
//
// HASHMAP & BTREEMAP
// hash -> tidak diurutkan (lebih cepat)
// btree-> diurutkan

#[test]
fn test_hash_map() {
    let mut map = HashMap::new();
    map.insert(String::from("key_1"), String::from("Aninditya"));
    map.insert(String::from("key_2"), String::from("Qamela"));
    map.insert(String::from("key_3"), String::from("Sauqiya"));

    let first = map.get("key_1");
    let third = map.get("key_3");

    println!("{}", first.unwrap());
    println!("{}", third.unwrap());
}

#[test]
fn test_btree_map() {
    let mut map = BTreeMap::new();
    map.insert(String::from("key_1"), String::from("Aninditya"));
    map.insert(String::from("key_3"), String::from("Sauqiya"));
    map.insert(String::from("key_2"), String::from("Qamela"));

    // disini balikannya berupa "entry", entry itu TUPPLE, dan TUPPLE tersebut isinya KEY, VALUE
    for enrtry in map {
        println!("{}: {}", enrtry.0, enrtry.1);
    }
}

// SET ***********************************************************************************************
// merupakan tipe data COLLECTION yang tidak boleh DUPLIKAT
//      jika dimasukkan (data yg sudah ada), maka tidak akan diterima
// SET tidak bisa di akses menggunakan INDEX
//
// HASHSET & BTREE SET
#[test]
fn test_hash_set() {
    let mut set = HashSet::new();
    set.insert(String::from("Aninditya"));
    set.insert(String::from("Qamela"));
    set.insert(String::from("Sauqiya"));
    set.insert(String::from("Sauqiya"));

    for name in set {
        println!("{}", name);
    }
}

#[test]
fn test_btree_set() {
    let mut set = BTreeSet::new();
    set.insert(String::from("Aninditya"));
    set.insert(String::from("Sauqiya"));
    set.insert(String::from("Qamela"));
    set.insert(String::from("Sauqiya"));

    for name in set {
        println!("{}", name);
    }
}
