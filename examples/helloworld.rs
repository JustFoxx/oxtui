use std::io::{BufWriter, Stdout};
use oxtui::TerminalBuilder;

fn main() {
    let terminal = TerminalBuilder::default()
        .title("Hello, World!")
        .raw_mode()
        .alternative_mode()
        .build();

}
