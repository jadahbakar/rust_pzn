use std::{array, vec};

fn main() {
    println!("Hello, world!");
}

// ITERATOR
// Setiap tipe data yang multiple seperti ARRAY, SLICE dan COLLECTION, memiliki fitur ITERATOR

#[test]
fn test_iterator() {
    let arr = [1, 2, 3, 4, 5, 6, 7];
    let mut iterator = arr.iter();

    while let Some(value) = iterator.next() {
        println!("{}", value)
    }

    for value in iterator {
        println!(" data: {}", value)
    }
}

// Pros using Iterator
// punya banyak METHOD untuk memanipulasi data
// kebanyakan METHOD di ITERATOR menggunakan CLOSURE sebagai Parameternya
//
#[test]
fn test_iterator2() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", vector);
    let sum: i32 = vector.iter().sum();
    println!("sum: {}", sum);
    let cnt: usize = vector.iter().count();
    println!("count: {}", cnt);
    let doubled: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);
    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", odd);
}
