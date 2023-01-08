pub mod request;
mod server;
mod thread_pool;

pub use server::Server;

#[cfg(test)]
mod tests {
    use super::*;
}
