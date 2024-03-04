use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    program_pack::Pack,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
    message::Message,
    transaction::VersionedTransaction,
};
use spl_token::state::Mint;
use bincode;
use bs58;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    send_tx_keypair: String,
    // mint_keypair: String,
    rpc_url_mainnet: String,
    send_keypair_mainnet: String,
    data: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let env = envy::from_env::<Env>().expect("配置环境变量失败");
    let send_keyPair = Keypair::from_base58_string(&env.send_tx_keypair);
    let client = RpcClient::new(env.rpc_url_mainnet.to_string());
    // let send_tx_account = Keypair::from_base58_string(&env.send_keypair_mainnet);
    // let 
    //get the data
    let data = &env.data  ;
    let data_a = bs58::decode(data).into_vec()?;
    let transaction: VersionedTransaction = bincode::deserialize(&data_a)?;
    let tx = VersionedTransaction::try_new( transaction.message , &[&send_keyPair]).unwrap();
    let result = client.send_and_confirm_transaction_with_spinner(&tx);
    println!("{:?}",result );
    // bincode::deserialize(&data_a.as_slice()).unwrap();
    // Message::deserialize(data_a);
    // let message: Instruction = serde_json::from_str(data)?;
    // println!("Message is {:?} ",data_a);
    // let tx: VersionedTransaction = serde_json::from_str(data)?;
    // let message: Transaction = ;
    // println!(" {:?} ", bs58::decode(data).into_vec().unwrap().as_slice());
    // let decode_bytes = bs58::decode(data).into_vec().unwrap();
    // println!("{:?}, {:?}",decode_bytes,decode_bytes.len());

    // let message =  Message::new(decode_bytes);
    // Transaction::new_unsigned(message);
    // solana_sdk::signature::Signature.from(decode_bytes);
    // let decode_bytes = data.from_base58().unwrap();
    // let decode_bytes = bs58::decode(data).into_vec().unwrap();
    // // let env = envy::from_env::<Env>()?;
    // println!(" env is {:?}, ", &env.rpc_url);
    // let signer_wallet = Keypair::from_base58_string(&env.signer_keypair);
    // let mint_account = Keypair::from_base58_string(&env.mint_keypair);
    // 

    // let decimals = 9;

    // let minimum_balance_for_rent_exemption =
    //     client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    // let create_account_instruction: Instruction = solana_sdk::system_instruction::create_account(
    //     &signer_wallet.pubkey(),
    //     &mint_account.pubkey(),
    //     minimum_balance_for_rent_exemption,
    //     Mint::LEN as u64,
    //     &spl_token::ID,
    // );

    // let initialize_mint_instruction: Instruction = spl_token::instruction::initialize_mint(
    //     &spl_token::ID,
    //     &mint_account.pubkey(),
    //     &signer_wallet.pubkey(),
    //     None,
    //     decimals,
    // )?;

    // let recent_blockhash = client.get_latest_blockhash()?;

    // let transaction: Transaction = Transaction::new_signed_with_payer(
    //     &[create_account_instruction, initialize_mint_instruction],
    //     Some(&signer_wallet.pubkey()),
    //     &[&mint_account, &signer_wallet],
    //     recent_blockhash,
    // );

    // client.send_and_confirm_transaction_with_spinner(&transaction)?;

    // println!(
    //     "SPL Token mint account with {} decimals created successfully:\n{}",
    //     decimals,
    //     mint_account.pubkey().to_string()
    // );

    Ok(())
}
