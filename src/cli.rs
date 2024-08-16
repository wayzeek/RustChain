use crate::blockchain::Blockchain;
use crate::errors::Result;
use clap::{arg, Command};


pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("rustchain")
        .version("0.1")
        .author("https://github.com/wayzeek")
        .about("A scratch-built blockchain in Rust, designed to explore and demonstrate the core principles of blockchain technology.")
        .subcommand(Command::new("print_chain").about("Print all the chain blocks"))
        .subcommand(
            Command::new("add_block")
            .about("Add a block to the blockchain")
            .arg(arg!(<DATA>"'the blockchain data'")),
        )
        .get_matches();


        if let Some(ref matches) = matches.subcommand_matches("add_block") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.cmd_add_block(String::from(c))?;
            }
            else {
                println!("You must add the data that you want in the block !");
            }
        }

        if let Some(_) = matches.subcommand_matches("print_chain") {
            self.cmd_print_chain();
        }
        
        Ok(())
    }

    fn cmd_add_block(&mut self, data : String) -> Result<()> {
        self.bc.add_block(data)
    }

    fn cmd_print_chain(&self) -> Result<()> {
        for b in self.bc.iter() {
            println!("{:?}", b);
        }
        Ok(())
    }
}