use std::io::{self, Write};

pub mod pos;

pub fn pause() {
    print!("Press Enter to continue...");
    // Flush stdout to guarantee the prompt prints immediately
    io::stdout().flush().unwrap();

    // Read input until a newline character is encountered
    let mut _dummy = String::new();
    io::stdin().read_line(&mut _dummy).unwrap();
}
