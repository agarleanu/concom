use std::io::stdout;

use concom::steps::run;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

fn main() -> std::io::Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;

    if let Err(err) = run() {
        eprintln!("{}", err);
    }

    Ok(())
}
