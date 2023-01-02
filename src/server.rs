use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fmt, io, net, thread, time};
use std::net::{TcpStream, TcpListener};
use std::io::prelude::*;

use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn bind<A: net::ToSocketAddrs>(addrs: A) -> Self {
        let listener = TcpListener::bind(addrs).unwrap();
        Server {
            listener,
        }
    }

    pub fn run(&self) -> Result<(), io::Error> {
        println!("shs HTTP server start!");
        // let term = Arc::new(AtomicBool::new(false));
        //
        // for sig in TERM_SIGNALS {
        //     flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term))?;
        //     flag::register(*sig, Arc::clone(&term))?;
        // }

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
        // while !term.load(Ordering::Relaxed) {
        //     println!("working...");
        //     thread::sleep(time::Duration::from_secs(1));
        // }
        // println!(
        //     "\nReceived kill signal. Wait 10 seconds, or hit Ctrl+C again to exit immediately."
        // );
        // thread::sleep(time::Duration::from_secs(1));

        println!("Exited cleanly");
        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
}
