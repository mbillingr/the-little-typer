use the_little_typer as tlt;

use std::io;
use std::io::Write;
use tlt::{
    basics::{Core, Ctx},
    rep::rep,
};

fn main() -> io::Result<()> {
    let ctx = Ctx::new();
    loop {
        let src = read_line()?;
        let core: Core = src.parse().unwrap();

        match rep(&ctx, &core) {
            Ok(out) => println!("{}", out),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn read_line() -> io::Result<String> {
    io::stdout().write(b"> ")?;
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}
