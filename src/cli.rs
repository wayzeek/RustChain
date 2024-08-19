use std::process::exit;
use clap::{arg, Command};
use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::transaction::Transaction;
use crate::wallet::Wallets;

pub struct Cli {

}

impl Cli {

    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("rustchain")
            .version("0.1")
            .author("https://github.com/wayzeek")
            .about("A scratch-built blockchain in Rust, designed to explore and demonstrate the core principles of blockchain technology.")
            .subcommand(Command::new("print_chain").about("Print all the blocks in the blockchain"))
            .subcommand(Command::new("create_wallet").about("Create a wallet"))
            .subcommand(Command::new("list").about("List all addresses"))
            .subcommand(Command::new("get_balance")
                .about("Get an address' balance")
                .arg(arg!(<ADDRESS>"'The target address to get balance of'"))
            )
            .subcommand(Command::new("create").about("Create new blochain")
                .arg(arg!(<ADDRESS>"'The address to send genesis block reward to' "))
            )
            .subcommand(
                Command::new("send")
                    .about("Send funds to an onther wallet")
                    .arg(arg!(<FROM>" 'Source wallet address'"))
                    .arg(arg!(<TO>" 'Destination wallet address'"))
                    .arg(arg!(<AMOUNT>" 'Amount of token to transfer'")),
            )
            .get_matches();


        if let Some(_) = matches.subcommand_matches("create_wallet") {
            let mut ws = Wallets::new()?;
            let address = ws.create_wallet();
            ws.save_all()?;
            println!("success: address {}", address);
        }

        if let Some(_) = matches.subcommand_matches("list") {
            let ws = Wallets::new()?;
            let addresses = ws.get_all_address();
            println!("addresses: ");
            for ad in addresses {
                println!("{}", ad);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                Blockchain::create_blockchain(address.clone())?;
                println!("create blockchain");
            }
            /*else {
                println!("Not printing testing lists...");
            }*/
        }
        if let Some(ref matches) = matches.subcommand_matches("get_balance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = Blockchain::new()?;
                let utxos = bc.find_UTXO(&address);
                let mut blance = 0;
                for out in utxos {
                    blance += out.value;
                }
                println!("Balance of '{}'; {} ", address, blance)
            }
            /*else {
                println!("Not printing testing lists...");
            }*/
        }
        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            }else {
                println!("from not supply!: usage");
                exit(1)
            };
            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            }else {
                println!("from not supply!: usage");
                exit(1)
            };
            let amount: i32 =   if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            }else {
                println!("from not supply!: usage");
                exit(1)
            };
            let mut bc = Blockchain::new()?;
            let tx = Transaction::new_UTXO(from, to, amount, &bc)?;
            bc.add_block(vec![tx])?;
            println!("success!");
            /*else {
                println!("Not printing testing lists...");
            }*/
        }
        if let Some(_) = matches.subcommand_matches("print_chain") {
            let bc = Blockchain::new()?;
            for b in &mut bc.iter() {
                println!("block: {:#?}", b);
            }
        }
        Ok(())
    }
}