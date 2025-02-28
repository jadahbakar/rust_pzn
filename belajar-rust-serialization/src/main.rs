use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Formatter;

fn main() {}

// STRUCT --------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all(
//     serialize = "SCREAMING_SNAKE_CASE",
//     deserialize = "SCREAMING_SNAKE_CASE"
// ))]
struct UserLoginRequest {
    username: String,
    password: String,
}

#[test]
fn test_create_json_for_user_login_request() {
    let login_request = UserLoginRequest {
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
    };
    // conver to json (serialized)
    let json = serde_json::to_string(&login_request).unwrap();
    println!("json: {}", json);
    // json to struct (deserialized)
    let login_result: UserLoginRequest = serde_json::from_str(&json).unwrap();
    println!("struct: {:?}", login_result);
}

// NESTED STRUCT --------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
struct AddressRequest {
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    email: String,
    address: AddressRequest,
}
#[test]
fn test_nested_struct() {
    let request = CreateUserRequest {
        username: "username".to_string(),
        password: "password".to_string(),
        email: "account@email.com".to_string(),
        address: AddressRequest {
            street: "ammt1".to_string(),
            city: "semarang".to_string(),
            state: "central java".to_string(),
            zip: "50192".to_string(),
        },
    };
    let json = serde_json::to_string(&request).unwrap();
    println!("json: {}", json);

    let result: CreateUserRequest = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}

// ARRAY --------------------------------------------------------
/*
Proses serialization bisa dilakukan untuk tipe array,
namun tidak bisa untuk deserialization karena ukuran Array di Rust harus ditentukan,
sedangkan mungkin kita tidak bisa menentukan jumlah Array dari format data lain
*/
#[test]
fn test_create_json_from_array() {
    let numbers = [10, 11, 12, 13, 14, 15];
    let json = serde_json::to_string(&numbers).unwrap();
    println!("{}", json)
}

// VECTOR --------------------------------------------------------
/*
Karena keterbatasan tipe data Array di Rust, oleh karena itu lebih banyak digunakan tipe data Vector untuk representasi Array di format data lain
Karena Vector ukurannya dinamis, jadi kita bisa dengan mudah melakukan serialization dan deserialization
*/

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    email: String,
    hobbies: Vec<String>,
}
#[test]
fn test_vector() {
    let user = User {
        username: "username".to_string(),
        email: "account@email.com".to_string(),
        hobbies: vec!["reading".to_string(), "codding".to_string()],
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);

    let return_from_json: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", return_from_json);
}

// OPTION --------------------------------------------------------
/*
Di Rust, terdapat Option yang bisa digunakan sebagai pembungkus data yang sifatnya tidak wajib atau optional
Rust Serialization juga mendukung tipe data Option
Jadi kita bisa dengan mudah memberitahu atribut mana yang optional dan atribut mana yang wajib
*/

#[derive(Debug, Serialize, Deserialize)]
struct UserId {
    username: String,
    email: String,
    hobbies: Vec<String>,
    phone: Option<String>,
}
#[test]
fn test_option() {
    let user = UserId {
        username: "username".to_string(),
        email: "account@email.com".to_string(),
        hobbies: vec!["reading".to_string(), "codding".to_string()],
        // phone: Some("089999919".to_string()),
        phone: None,
    };
    let to_json = serde_json::to_string(&user).unwrap();
    println!("{}", to_json);

    let to_struct: UserId = serde_json::from_str(&to_json).unwrap();
    println!("{:#?}", to_struct.phone);
}

// MAP ---------------------------------------------------------------------
/*
Saat kita menggunakan Struct, kita wajib tahu semua nama atribut ketika melakukan proses serialization dan deserialization
Namun, kadang ada kasus dimana kita tidak tahu nama-nama atribut, atau bahkan bisa berubah-ubah
Pada kasus seperti ini, biasanya kita akan menggunakan tipe data Map yang bisa menggunakan key yang dinamis
*/

#[test]
fn test_map() {
    let mut values = HashMap::new();
    values.insert("satu", 1);
    values.insert("dua", 2);
    values.insert("tiga", 3);

    let to_json = serde_json::to_string(&values).unwrap();
    println!("{}", to_json);

    let result: HashMap<String, i32> = serde_json::from_str(&to_json).unwrap();
    println!("{:#?}", result);
}

// ATTRIBUTE ----------------------------------------------------------------------
// Container Attribute
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(
    serialize = "SCREAMING_SNAKE_CASE",
    deserialize = "SCREAMING_SNAKE_CASE"
))]
struct UserLoginRequest2 {
    username: String,
    password: String,
}

// Field Attribute
#[derive(Debug, Deserialize, Serialize)]
struct UserId2 {
    username: String,
    password: String,
    email: String,
    #[serde(rename = "alamat")]
    address: AddressRequest,
}

// ENUM --------------------------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
struct Users {
    username: String,
    email: String,
    hobbies: Vec<String>,
    #[serde(rename = "handphone")]
    phone: Option<String>,
    gender: Gender,
}

#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

#[test]
fn test_enum() {
    let user = Users {
        username: "dedy".to_string(),
        email: "account@email.com".to_string(),
        hobbies: vec!["reading".to_string(), "codding".to_string()],
        phone: Some("08999919".to_string()),
        gender: Gender::Male,
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);

    let result: Users = serde_json::from_str(&json).unwrap();
    println!("{:?}", result);
}

// ENUM DATA --------------------------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
struct Users2 {
    username: String,
    email: String,
    hobbies: Vec<String>,
    phone: Option<String>,
    gender: Gender,
    payment: Payment,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Payment {
    CreditCard {
        card_number: String,
        expiration: String,
    },
    BankAccount {
        account_number: String,
        bank_name: String,
    },
}

#[test]
fn test_enum_data() {
    let user = Users2 {
        username: "dedy".to_string(),
        email: "account@email.com".to_string(),
        hobbies: vec!["reading".to_string(), "codding".to_string()],
        phone: Some("08999919".to_string()),
        gender: Gender::Male,
        payment: Payment::BankAccount {
            account_number: "1234567890".to_string(),
            bank_name: "Bank BCA".to_string(),
        },
    };

    let to_json = serde_json::to_string(&user).unwrap();
    let to_struct: Users2 = serde_json::from_str(&to_json).unwrap();

    println!("{}", to_json);
    println!("{:?}", to_struct);
}

// CHRONO ----------------------------------------------------------------------------------------------------
/*
Chrono sendiri memiliki module untuk membantu melakukan Serde, namun terbatas hanya untuk tipe data DateTime<Utc>
cargo add chrono --features serde
*/
#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    name: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    updated_at: DateTime<Utc>,
}

#[test]
fn test_chrono() {
    let category = Category {
        id: "1234".to_string(),
        name: "Dedy".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let to_json = serde_json::to_string(&category).unwrap();
    let to_struc: Category = serde_json::from_str(&to_json).unwrap();

    println!("{}", to_json);
    println!("{:?}", to_struc);
}

// CUSTOM SERIALIZATION ----------------------------------------------------------------------------------------------------
#[derive(Serialize, Debug)]
struct Admin {
    id: String,
    name: Name,
}

#[derive(Debug)]
struct Name {
    first: String,
    last: String,
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{} {}", self.first, self.last).as_str())
    }
}

#[test]
fn test_custom_serialize() {
    let admin = Admin {
        id: "12345".to_string(),
        name: Name {
            first: "john".to_string(),
            last: "doe".to_string(),
        },
    };

    let to_json = serde_json::to_string(&admin).unwrap();
    println!("{}", to_json);
}

// CUSTOM DESERIALIZATION ----------------------------------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
struct Admin2 {
    id: String,
    name: Name2,
}

#[derive(Debug)]
struct Name2 {
    first: String,
    last: String,
}

struct NameVisitor;

impl Serialize for Name2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{} {}", self.first, self.last).as_str())
    }
}

impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name2;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting name string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let result: Vec<&str> = v.split(" ").collect();
        if result.len() != 2 {
            return Err(Error::custom("Expecting first and last name"));
        }

        Ok(Name2 {
            first: result[0].to_string(),
            last: result[1].to_string(),
        })
    }
}

impl<'de> Deserialize<'de> for Name2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(NameVisitor)
    }
}

#[test]
fn test_custom_serialization() {
    let admin = Admin2 {
        id: "1234".to_string(),
        name: Name2 {
            first: "John".to_string(),
            last: "Doe".to_string(),
        },
    };

    let to_json = serde_json::to_string(&admin).unwrap();
    let to_struc: Admin2 = serde_json::from_str(&to_json).unwrap();

    println!("{}", to_json);
    println!("{:?}", to_struc);
}

// SERDE MODULE ----------------------------------------------------------------------------------------------------
/*
Pada beberapa kasus, kadang kita butuh membuat Serialize dan Deserialize untuk tipe data yang terdapat di Crate lain
Dan misal sayangnya, Crate tersebut bukanlah milik kita, contoh misal tipe data NaiveDateTime di library Chrono
Kita tidak bisa membuat implementasi Serialize dan Deserialize karena NaiveDateTime tidak terdapat di Crate kita
Namun untungnya, Serde menyediakan cara membuat Serialize dan Deserialize dengan cara membuatmethod
Cara penggunaannya cukup gunakan attribute serde dan with seperti pada bab Chrono yang sudah kita bahas
*/

pub mod pzn {
    pub mod serde {
        pub mod chrono {
            pub mod to_milsec {
                use chrono::{DateTime, NaiveDateTime};
                use serde::de::{Error, Visitor};
                use serde::{Deserializer, Serializer};

                pub fn serialize<S>(
                    datetime: &NaiveDateTime,
                    serializer: S,
                ) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let ms = datetime.and_utc().timestamp_millis();
                    serializer.serialize_i64(ms)
                }

                struct NaiveDateTimeVisitor;

                impl<'de> Visitor<'de> for NaiveDateTimeVisitor {
                    type Value = NaiveDateTime;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("expecting u64")
                    }

                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        let datetime = DateTime::from_timestamp_millis(v as i64)
                            .unwrap()
                            .naive_utc();
                        Ok(datetime)
                    }
                }

                pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    deserializer.deserialize_u64(NaiveDateTimeVisitor)
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Penjualan {
    id: String,
    name: String,
    #[serde(with = "crate::pzn::serde::chrono::to_milsec")]
    created_at: NaiveDateTime,
    #[serde(with = "crate::pzn::serde::chrono::to_milsec")]
    updated_at: NaiveDateTime,
}

#[test]
fn test_penjualan() {
    let penjualan = Penjualan {
        id: "1234".to_string(),
        name: "Dedy".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let to_json = serde_json::to_string(&penjualan).unwrap();
    let to_struc: Category = serde_json::from_str(&to_json).unwrap();

    println!("{}", to_json);
    println!("{:?}", to_struc);
}
