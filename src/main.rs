use std::io::stdout;

use concom::steps::run;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

fn main() -> std::io::Result<()> {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

    if let Err(err) = run() {
        eprintln!("{}", err);
    }

    Ok(())
}
