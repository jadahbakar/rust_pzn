fn main() {
    println!("Hello, world!");
}

// ATTRIBUTE
// Attribute merupakan cara menambah metadata (informasi tambahan) ke kode yang kita buat
// Syntax Attribute di Rust menggunakan tanda #[NamaAttribute]
//
// Derive ATTRIBUTE
// membuat implementasi TRAIT secara otomatis
//  tidak semua TRAIT bisa otomatis dibuat implementasinya, hanya yang sudah ditentukan

#[derive(Debug)]
struct Company {
    name: String,
    location: String,
}

#[test]
fn test_attribute() {
    let company = Company {
        name: "DotCode".to_string(),
        location: "Semarang".to_string(),
    };
    println!("{:#?}", company)
}

/*
Untuk melihat hasil kode yang dibuat, kita bisa gunakan cargo-expand
    https://github.com/dtolnay/cargo-expand
Silahkan install terlebih dahulu menggunakan perintah :
    cargo install cargo-expand
Lalu untuk melihat hasil kode yang dibuat, kita bisa gunakan perintah
    cargo expand nama_module, atau untuk testing function
    cargo expand --tests nama_module

*/
