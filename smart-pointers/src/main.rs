pub mod deref;
pub mod drop;
pub mod reference_counted;
pub mod smartpointers;

fn main() {
    smartpointers::smartpointers();
    deref::deref_trait();
    drop::drop_trait();
    reference_counted::reference_counted();
}
