use serde::Serialize;
use validator::{Validate, ValidateArgs, ValidationErrors};

fn main() {}

#[derive(Debug, Validate)]
struct LoginRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 character"
    ))]
    username: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Password must be between 3 and 20 character"
    ))]
    password: String,
}

#[derive(Debug, Validate)]
struct AddressRequest {
    #[validate(length(min = 3, max = 100))]
    street: String,
    #[validate(length(min = 3, max = 100))]
    city: String,
    #[validate(length(min = 3, max = 100))]
    country: String,
}

#[derive(Debug, Validate)]
struct RegisterUserRequest {
    #[validate(length(min = 3, max = 20))]
    username: String,
    #[validate(length(min = 3, max = 20))]
    password: String,
    #[validate(length(min = 3, max = 100))]
    name: String,
    #[validate(nested)]
    address: AddressRequest,
}
#[test]
fn test_nested_struct_success() {
    let request = RegisterUserRequest {
        username: "eko".to_string(),
        password: "eko".to_string(),

        name: "eko".to_string(),
        address: AddressRequest {
            street: "street".to_string(),
            city: "city".to_string(),
            country: "country".to_string(),
        },
    };

    assert!(request.validate().is_ok())
}

#[test]
fn test_nested_struct_error() {
    let request = RegisterUserRequest {
        username: "".to_string(),
        password: "eko".to_string(),

        name: "".to_string(),
        address: AddressRequest {
            street: "".to_string(),
            city: "".to_string(),
            country: "".to_string(),
        },
    };

    let errors: ValidationErrors = request.validate().err().unwrap();
    println!("{:?}", errors.errors());
}

#[test]
fn test_validate_success() {
    let login = LoginRequest {
        username: "eko".to_string(),
        password: "password".to_string(),
    };
    assert!(login.validate().is_ok())
}

#[test]
fn test_validate_failed() {
    let login = LoginRequest {
        username: "ek".to_string(),
        password: "password".to_string(),
    };
    assert!(login.validate().is_err());

    let errors: ValidationErrors = login.validate().err().unwrap();
    println!("{:?}", errors.errors());
}

// COLLECTION -------------------------------------------------------------------------------------------
/*
    Syaratnya jika tipe data di dalam Collection tersebut adalah Struct, maka Struct nya harus implement Validate dan juga serde::Serialize
    Sama seperti Nested Struct, secara default isi dari Collection tidak akan divalidasi, kecuali kita gunakan jenis validation nested
*/

#[derive(Debug, Validate)]
struct Product {
    #[validate(length(min = 3, max = 100))]
    id: String,
    #[validate(length(min = 3, max = 100))]
    name: String,
    #[validate(nested, length(min = 1))]
    variants: Vec<ProductVariant>,
}
#[derive(Debug, Validate, Serialize)]
struct ProductVariant {
    #[validate(length(min = 3, max = 100))]
    name: String,
    #[validate(range(min = 1, max = 1000000000))]
    price: i32,
}

#[test]
fn test_validate_vector_success() {
    let request = Product {
        id: "product-1".to_string(),
        name: "product-1".to_string(),
        variants: vec![
            ProductVariant {
                name: "variant 1".to_string(),
                price: 1000,
            },
            ProductVariant {
                name: "variant 2".to_string(),
                price: 2000,
            },
        ],
    };

    assert!(request.validate().is_ok())
}

#[test]
fn test_validate_vector_error() {
    let request = Product {
        id: "product-1".to_string(),
        name: "product-1".to_string(),
        variants: vec![
            ProductVariant {
                name: "".to_string(),
                price: -1000,
            },
            ProductVariant {
                name: "".to_string(),
                price: -2000,
            },
        ],
    };

    assert!(request.validate().is_err());
    let errors: ValidationErrors = request.validate().err().unwrap();
    println!("{:?}", errors.errors());
}

// CUSTOM VALIDATION ----------------------------------------------------------------------------
/*
    membuat validasi di aplikasi, kadang kita butuh membuat validasi yang baru yang mungkin tidak tersedia di library Validator
    Kita bisa menggunakan jenis validasi custom, dimana kita bisa buat function untuk melakukan validasinya
    Function untuk melakukan validasinya harus mengembalikan Result<(), ValidationError>
*/

pub mod pzn {
    pub mod validator {
        use std::borrow::Cow;
        use validator::ValidationError;
        pub fn not_blank(value: &str) -> Result<(), ValidationError> {
            if value.trim().is_empty() {
                return Err(ValidationError::new("not_blank")
                    .with_message(Cow::from("value cannot be blank")));
            }
            Ok(())
        }

        pub fn password_equal_confirm_pasword(
            todo!()

        )
    }
}

#[derive(Debug, Validate)]
struct CreateCategoryRequest {
    #[validate(custom(function = "crate::pzn::validator::not_blank"))]
    id: String,
    #[validate(custom(function = "crate::pzn::validator::not_blank"))]
    name: String,
}

#[test]
fn test_custom_validation() {
    let category = CreateCategoryRequest {
        id: "".to_string(),
        name: "".to_string(),
    };

    let erros: ValidationErrors = category.validate().err().unwrap();
    println!("{:?}", erros.errors());
}

// STRUC LEVEL VALIDATION ----------------------------------------------------------------------------
#[derive(Debug, Validate)]
struct RegisterUser {
    #[validate(length(min = 3, max = 20))]
    username: String,

    #[validate(length(min = 3, max = 20))]
    password: String,
    #[validate(length(min = 3, max = 20))]
    confirm_password: String,

    #[validate(length(min = 3, max = 100))]
    name: String,
    #[validate(nested)]
    address: AddressRequest,
}
