fn main() {
    println!("Hello, Rust DateTime !");
}

/*
Chrono memiliki dua jenis tipe data waktu, tipe data waktu yang memiliki timezone dan yang tidak memiliki timezone
Sekarang kita akan bahas dulu tipe data yang tidak memiliki timezone
Dimulai dari tipe data Date
Date adalah tipe data waktu tanggal bulan dan tahun
Date direpresentasikan dengan struct NaiveDate di Chrono
*/

#[cfg(test)]
mod test {

    use chrono::{
        DateTime, Datelike, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime,
        TimeZone, Timelike, Utc,
    };
    use chrono_tz::Asia::{self, Jakarta};

    #[test]
    fn test_naive_date() {
        let date = NaiveDate::from_ymd_opt(2025, 2, 27).unwrap();
        println!("{}", date.year());
        println!("{}", date.month());
        println!("{}", date.day());
    }

    // DURATION ---------------------------------------------------------------------
    /*
    Semua tipe data waktu di Chrono adalah Immutable, artinya tidak bisa berubah
    Jadi tidak ada method yang bisa kita gunakan untuk mengubah waktu pada object yang sudah dibuat
    Namun Chrono menyediakan operator Add (+) dan Sub (-) yang bisa kita gunakan untuk menambah/mengurangi waktu
    Namun operator ini hanya bisa digunakan dengan tipe data Duration, yaitu alias untuk TimeDelta
    */

    #[test]
    fn test_duration() {
        let date = NaiveDate::from_ymd_opt(2025, 02, 27).unwrap();
        let new_date = date + Duration::days(10);
        println!("{}", new_date);
    }

    // TIME ---------------------------------------------------------------------
    /*
    Chrono mendukung tipe data Time (jam menit detik nanosecond) tanpa timezone menggunakan struct NaiveTime
    */

    #[test]
    fn test_time() {
        let time = NaiveTime::from_hms_milli_opt(11, 26, 00, 500).unwrap();
        println!("{}", time.hour());
        println!("{}", time.minute());
        println!("{}", time.second());
        println!("{}", time.nanosecond());
    }

    // DATETIME ---------------------------------------------------------------------
    #[test]
    fn test_date_time() {
        let date_time = NaiveDate::from_ymd_opt(2025, 02, 27)
            .unwrap()
            .and_hms_opt(13, 24, 20)
            .unwrap();

        println!("{}", date_time.year());
        println!("{}", date_time.month());
        println!("{}", date_time.day());

        println!("{}", date_time.hour());
        println!("{}", date_time.minute());
        println!("{}", date_time.second());
        println!("{}", date_time.nanosecond());

        let date_time2 = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 02, 27).unwrap(),
            NaiveTime::from_hms_opt(13, 27, 20).unwrap(),
        );

        println!("{}", date_time2.date());
        println!("{}", date_time2.time());
    }

    // TIMEZONE ---------------------------------------------------------------------
    #[test]
    fn test_fixed_offset() {
        let utc_date_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 02, 27).unwrap(),
            NaiveTime::from_hms_opt(13, 27, 20).unwrap(),
        );

        let asia_jakarta = FixedOffset::east_opt(7 * 3600).unwrap();
        let asia_jakarta_date_time = asia_jakarta.from_utc_datetime(&utc_date_time);

        println!("{}", utc_date_time);
        println!("{}", asia_jakarta_date_time);
    }

    // Chrono TimeZone ---------------------------------------------------------------------
    #[test]
    fn test_time_zone() {
        let utc_date_time = Utc::now();
        let asia_jakarta_tz = Asia::Jakarta.from_utc_datetime(&utc_date_time.naive_utc());

        println!("{}", utc_date_time);
        println!("{}", asia_jakarta_tz);

        let local_pc_tz = Local::now();
        let asia_jakarta_tz = Asia::Jakarta
            .from_local_datetime(&local_pc_tz.naive_local())
            .unwrap();

        println!("{}", local_pc_tz);
        println!("{}", asia_jakarta_tz);
    }

    // PARSING ---------------------------------------------------------------------
    #[test]
    fn test_parsing_date() {
        let str = String::from("2025-02-27 14:02:00 +0700");
        let date_time = DateTime::parse_from_str(&str, "%Y-%m-%d %H:%M:%S %z").unwrap();

        println!("{}", date_time.year());
        println!("{}", date_time.month());
        println!("{}", date_time.day());

        println!("{}", date_time.hour());
        println!("{}", date_time.minute());
        println!("{}", date_time.second());
        println!("{}", date_time.nanosecond());
        println!("{}", date_time.timezone());
    }

    // FORMAT ---------------------------------------------------------------------
    #[test]
    fn test_format() {
        let now = Local::now();
        let formatted = now.format("%Y-%m-%d %H:%M:%S %z").to_string();

        println!("{}", formatted);
    }
}
