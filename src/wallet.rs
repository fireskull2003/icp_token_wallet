use dfinity_sdk::{Client, Icrc2Token};
use hex::{FromHex, ToHex};
use serde::{Serialize, Deserialize};

pub struct TokenWallet {
    client: Client,
    account_id: String,
    private_key: String,
}

impl TokenWallet {
    pub fn new(account_id: String, private_key: String) -> Self {
        let client = Client::new("https://api.dfinity.network/icp-api");
        TokenWallet { client, account_id, private_key }
    }

    pub fn get_balance(&self) -> Result<u64, String> {
        let token_id = "your_token_id_here"; // Replace with your IRCRC2 token ID
        let token = Icrc2Token::new(token_id);
        let balance = self.client.get_token_balance(&self.account_id, &token)?;
        Ok(balance)
    }

    pub fn send_tokens(&self, to_account_id: String, amount: u64) -> Result<(), String> {
        let token_id = "your_token_id_here"; // Replace with your IRCRC2 token ID
        let token = Icrc2Token::new(token_id);
        let tx_id = self.client.send_token_transfer(
            &self.account_id,
            &to_account_id,
            &token,
            amount,
            &self.private_key,
        )?;
        println!("Transaction ID: {}", tx_id);
        Ok(())
    }
    pub fn receive_tokens(&self) -> Result<(), String> {
        let mut subscription = self.client.subscribe_to_transactions(&self.account_id)?;
        loop {
            let tx = subscription.next()?;
            if let Some(tx) = tx {
                if tx.token_id == "y2yeb-nkdsv-4bdm7-kmlzk-himxq-7oxyy-5atgh-lp4n4-hdkuo-4guly-aae" { // Replace with your IRCRC2 token ID
                    let amount = tx.amount;
                    println!("Received {} tokens", amount);
                    // Update the wallet balance
                    self.client.update_token_balance(&self.account_id, &tx.token_id, amount)?;
                }
            }
        }
    }
}
fn main() {
    let wallet = TokenWallet::new("your_account_id_here".to_string(), "your_private_key_here".to_string());
    // Use the wallet instance to call the functions
    wallet.get_balance().unwrap();
    wallet.send_tokens("to_account_id_here".to_string(), 10).unwrap();
    wallet.receive_tokens().unwrap();
}