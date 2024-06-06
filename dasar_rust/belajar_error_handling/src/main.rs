use std::result;

fn main() {
    println!("Hello, world!");
}

// ERROR HANDLING
// Rust membagi 2 -> RECOVERABLE dan UNRECOVERABLE (panic!)
//  Tidak punya Exception
//
// UNRECOVERABLE
fn connect_database(host: Option<String>) {
    match host {
        Some(host) => println!("Connecting to database at {}", host),
        None => panic!("No database host provided"),
    }
}

#[test]
fn test_db_connect() {
    connect_database(None)
}

// RECOVERABLE
// Sama seperti ENUM OPTION, Rust menyediakan ENUM RESULT
//      misal kita membuat Function yang bisa mengembalikan sukses atau gagal,
//      kita bisa buat Function  dengan return value ENUM RESULT
// ENUM RESULT memiliki 2 nilai -> Ok(T) dan Err(E)

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(host),
        None => Err("No cache host provided".to_string()),
    }
}
#[test]
fn test_recoverable_error() {
    // let cache = connect_cache(None);
    let cache = connect_cache(Some("localhost".to_string()));

    match cache {
        Ok(host) => println!("Connecting to cache at {}", host),
        Err(err) => println!("Error connecting to cache at {}", err),
    }
}

// OPERATOR ?
// Saat menggunakan Recoverable Error, kadang kita sering memanggil beberapa jenis function yang menghasilkan Result,
//      lalu ingin mengecek, jika Err maka kita ingin langsung mengembalikan error itu secara langsung
// Jika melakukan manual menggunakan Pattern Matching, kadang menyulitkan
// Kita bisa menggunakan ? Operator, yang secara otomatis bisa mengembalikan Result jika memang Err

fn connect_db(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(host),
        None => Err("No db host provided".to_string()),
    }
}
fn connect_redis(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(host),
        None => Err("No redis host provided".to_string()),
    }
}

fn connect_app(host: Option<String>) -> Result<String, String> {
    // let connect_db = connect_db(host.clone());
    // match connect_db {
    //     Ok(_) => {}
    //     Err(err) => {
    //         return Err(err);
    //     }
    // }
    // let connect_redis = connect_redis(host.clone());
    // match connect_redis {
    //     Ok(_) => {}
    //     Err(err) => {
    //         return Err(err);
    //     }
    // }

    connect_db(host.clone())?; // ---apabila result error, langsung ke kembalikan ke Result yg error (atas)
    connect_redis(host.clone())?; // ---apabila result error, langsung ke kembalikan ke Result yg error (atas)

    Ok("Connnected to App".to_string())
}

#[test]
fn test_connect_app() {
    let result = connect_app(Some("localhost".to_string()));
    match result {
        Ok(msg) => println!("Success connect with message: {}", msg),
        Err(err) => println!("Error connect, with message: {}", err),
    }
}
