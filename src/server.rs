use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fmt, io, thread, time};

use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn run(self) -> Result<(), io::Error> {
        println!("shs HTTP server start!");
        let n = 0;
        let term = Arc::new(AtomicBool::new(false));

        for sig in TERM_SIGNALS {
            flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term))?;
            flag::register(*sig, Arc::clone(&term))?;
        }

        while !term.load(Ordering::Relaxed) {
            println!("http server working...");
            thread::sleep(time::Duration::from_secs(1));
        }
        println!(
            "\nReceived kill signal. Wait 10 seconds, or hit Ctrl+C again to exit immediately."
        );
        thread::sleep(time::Duration::from_secs(1));

        println!("Exited cleanly");
        Ok(())
    }
}
