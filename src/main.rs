use std::io::stdout;

use concom::steps::run;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

fn main() -> std::io::Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;

    match run() {
        Ok(commit) => {
            println!("{}", commit)
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    }

    Ok(())
}
