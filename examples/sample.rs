use shs::Server;
use std::io;

fn main() -> std::io::Result<()> {
    // Shs::new(
    //
    // ).bind()?.run()
    Server::new().run()
}
