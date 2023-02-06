use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{fmt, io, net, thread, time};

use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;

use crate::thread_pool::ThreadPool;

pub struct ServerInner {
    listener: Mutex<TcpListener>,
}

pub struct Server {
    inner: Arc<ServerInner>,
}

impl Server {
    pub fn bind<A: net::ToSocketAddrs>(addrs: A) -> Self {
        let listener = TcpListener::bind(addrs).unwrap();
        Server {
            inner: Arc::new(ServerInner {
                listener: Mutex::new(listener),
            }),
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        println!("shs HTTP server start!");

        // let term = Arc::new(AtomicBool::new(false));
        //
        // for sig in TERM_SIGNALS {
        //     flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term))?;
        //     flag::register(*sig, Arc::clone(&term))?;
        // }

        let pool = ThreadPool::new(4);

        for stream in self.inner.listener.lock().unwrap().incoming() {
            let stream = stream.unwrap();
            let local_self = self.inner.clone();
            pool.execute(move || {
                local_self.handle_connection(stream);
            });
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
}

impl ServerInner {
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let get = b"GET / HTTP/1.1\r\n";
        let post = b"POST / HTTP/1.1\r\n";
        let put = b"PUT / HTTP/1.1\r\n";
        let delete = b"DELETE / HTTP/1.1\r\n";

        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=UTF-8\r\n\r\n", "public/index.html")
        } else if buffer.starts_with(post) {
            ("HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=UTF-8\r\n\r\n", "public/post.html")
        } else if buffer.starts_with(put) {
            ("HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=UTF-8\r\n\r\n", "public/put.html")
        } else if buffer.starts_with(delete) {
            ("HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=UTF-8\r\n\r\n", "public/delete.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\ncontent-type: text/html; charset=UTF-8\r\n\r\n", "404.html")
        };

        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
}
