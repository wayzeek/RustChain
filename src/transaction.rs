
use crate::errors::Result;
use crate::blockchain::Blockchain;
use crypto::{digest::Digest, sha2::Sha256};
use failure::format_err;
use serde::{Serialize, Deserialize};
use log::error;
use crate::txio::{TXInput, TXOutput};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

impl Transaction {

    pub fn new_utxo(from: &str, to: &str, amount: i32, bc: &Blockchain) -> Result<Transaction> {
        let mut vin = Vec::new();
        let acc_v = bc.find_spendable_outputs(from, amount);

        if acc_v.0 < amount {
            error!("Not enough balance !");
            return Err(format_err!("Not enough balance : current balance {}", acc_v.0));
        }

        for tx in acc_v.1 {
            for out in tx.1 {
                let input = TXInput {
                    txid : tx.0.clone(),
                    vout : out,
                    script_sig: String::from(from),
                };
                vin.push(input);
            }
        }

        let mut vout = vec![TXOutput {
            value : amount,
            script_pub_key: String::from(to),
        }];

        if acc_v.0 > amount {
            vout.push( TXOutput {
                value: acc_v.0 - amount,
                script_pub_key: String::from(from),
            });
        }

        let mut tx = Transaction {
            id: String::new(),
            vin,
            vout,
        };
        tx.set_id()?;
        Ok(tx)
    }

    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction> {
        if data.is_empty() {
            data += &format!("Reward to '{}'", to);
        }
        let mut tx = Transaction {
            id: String::new(),
            vin : vec![TXInput {
                txid: String::new(),
                vout : -1,
                script_sig: data,
            }],
            vout: vec![TXOutput {
                value: 100,
                script_pub_key: to,
            }]
        };
        tx.set_id()?;
        Ok(tx)
    }

    fn set_id(&mut self) -> Result<()> {
        let mut hasher = Sha256::new();
        let data = bincode::serialize(&self)?;
        hasher.input(&data[..]);
        self.id = hasher.result_str();
        Ok(())
    }

    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].txid.is_empty() && self.vin[0].vout == -1
    }
}
