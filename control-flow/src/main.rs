pub mod for_control;
pub mod if_else;
pub mod if_while_let;
pub mod labeled_blocks;
pub mod let_else;
pub mod loop_control;
pub mod while_control;

fn main() {
    if_else::if_else();
    loop_control::loop_control();
    while_control::while_control();
    for_control::for_control();
    if_while_let::if_while_let();
    let_else::let_else();
    labeled_blocks::labeled_blocks();
}
