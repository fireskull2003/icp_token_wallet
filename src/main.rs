mod wallet;
use token_wallet::TokenWallet;

fn main() {
    let account_id = "rohit".to_string();
    let private_key = "y2yeb-nkdsv-4bdm7-kmlzk-himxq-7oxyy-5atgh-lp4n4-hdkuo-4guly-aae".to_string();
    let wallet = TokenWallet::new(account_id, private_key);

    println!("Wallet balance: {}", wallet.get_balance().unwrap());

    let to_account_id = "recipient_account_id_here".to_string();
    let amount = 10;
    wallet.send_tokens(to_account_id, amount).unwrap();

    wallet.receive_tokens().unwrap();
}