pub mod async_basics;
pub mod async_runtimes;
pub mod async_vs_threads;
pub mod tokio_chat;

use crate::async_basics::async_basics;
use crate::async_runtimes::async_runtimes;
use crate::async_vs_threads::async_vs_threads;
use crate::tokio_chat::tokio_chat;

fn main() {
    async_vs_threads();
    async_basics();
    async_runtimes();
    tokio_chat();
}
