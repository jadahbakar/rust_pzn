use chrono::{self, Local, Timelike};

#[cfg(feature = "greet")]
pub fn say_greetings() -> String {
    let hour = Local::now().hour();

    let greet = match hour {
        0..=10 => "Morning",
        11..=14 => "Afternoon",
        15..=20 => "Evening",
        _ => "Night",
    };
    format!("{}", greet)
}

#[cfg(feature = "jam")]
pub fn get_time() -> String {
    let jam = Local::now();
    format!("{:?}", jam)
}
