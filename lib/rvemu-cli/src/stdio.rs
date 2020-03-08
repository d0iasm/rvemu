use console::Term;

/// Input a message from the console in a host computer.
pub fn stdin() {
    let term = Term::stdout();
    match term.read_key() {
        Ok(c) => {
            println!("something comes! {:#?}", c);
            // TODO: Change csr states in order to transmit an interrupt occurs.
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub fn stdout() {}
