mod transaction;
mod txio;
mod wallet;

mod block;
mod blockchain;

mod errors;

mod cli;

use crate::cli::Cli;
use crate::errors::Result;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())

}
