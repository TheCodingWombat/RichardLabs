// #![feature(register_tool)]
// #![register_tool(myclippy)]
#[warn(
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::todo,
    clippy::unwrap_used,
    clippy::panic
)]
mod auxiliary;

pub struct Twox64Concat;

fn main() {
    println!("Hello, world!");

    let a = 3;

    if !true {
        print!("a");
    } else {
        print!("b");
    }
}
