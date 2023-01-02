use shs::Server;
use std::io;

fn main() -> std::io::Result<()> {
    // Shs::new(
    //
    // ).bind()?.run()
    Server::bind(("127.0.0.1", 8080)).run()
}
