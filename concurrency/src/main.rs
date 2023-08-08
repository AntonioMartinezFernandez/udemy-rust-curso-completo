mod threads;
mod threads_communication;
mod threads_multiple_accessing;

use crate::threads::main_threads;
use crate::threads_communication::main_threads_communication;
use crate::threads_multiple_accessing::main_threads_multiple_accessing;

fn main() {
    main_threads();
    main_threads_communication();
    main_threads_multiple_accessing();
}
