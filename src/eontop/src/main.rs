use crate::command::Eontop;

mod command;

fn main() {
    let eontop = Eontop::new();
    eontop.run();
}
