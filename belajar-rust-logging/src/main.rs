fn main() {
    println!("Hello, Log!");
}

#[cfg(test)]
mod test {

    // LEVEL ------------------------------------------------------------
    /*
    Untuk melakukan log, kita bisa menggunakan macro log!(level, log)
    https://docs.rs/log/0.4.22/log/macro.log.html
    Atau kita bisa gunakan shortcut macro
    error!(log) untuk level error
    warn!(log) untuk level warn
    info!(log) untuk level info
    debug!(log) untuk level debug
    trace!(log) untuk level trace
    */

    use log::{debug, error, info, trace, warn};

    #[test]
    fn test_log_level() {
        error!("this is an error");
        warn!("this is and warn");
        info!("this is an info");
        debug!("this is a debug");
        trace!("this is a trace");
    }

    // SIMPLE LOG ------------------------------------------------------------
    // set environment on terminal
    // ❯ export RUST_LOG=info
    // run it
    // ❯ cargo test -- --show-output

    #[test]
    fn test_simple_log() {
        env_logger::init();

        error!("this is an error");
        warn!("this is and warn");
        info!("this is an info");
        debug!("this is a debug");
        trace!("this is a trace");
    }
    // Complex Logger ------------------------------------------------------------
    #[test]
    fn test_complex_log() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        error!("this is an error");
        warn!("this is and warn");
        info!("this is an info");
        debug!("this is a debug");
        trace!("this is a trace");
    }
}
