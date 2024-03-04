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
    signer_keypair: String,
    // mint_keypair: String,
    rpc_url_mainnet: String,
    send_keypair_mainnet: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let env = envy::from_env::<Env>().expect("配置环境变量失败");
    let client = RpcClient::new(env.rpc_url_mainnet.to_string());
    let send_tx_account = Keypair::from_base58_string(&env.send_keypair_mainnet);
    // let 
    let data = "31s9owhAPGMTXqJvmEmv69FdUNH5pPMP6cVb3CUa6Tde2fXVNWncPx7hMD6cDXvJrNSyF9pc1vKqgvV7S4PB1gvJrCczDCNsTsSbHLjnvUWUPcHss3bsjZ5EQ9Xx1fT1NdQi3aMa4TKvz3iZgP5GRTisZ5KcGjZS34MqPqEaMGEFf8STpWXD9QF6P8QapM4jU9eawjAf8Mvsqb9nQNrp7qdyKyk2AKoeia52DLsBNGei7ARhzJdBomLoPsp7NLfQrpEwupUf8Ms6cDJnz9KgSDwtKhvx9PfXdbSGKuBgunL6pKMXgDGMhcXdcWkfjL1N6aXtzXredTF6RSneBnv59MFsnz7QQnHWbpNr7mv5rcsAHsW7qKUnjBqqfgioCRDyFzKeJTCvqtoUYXW7jgAej3EFxDzUfxLC1f7HQxdSUQzEwVj8sQHMXN3thYSDZkqZw4QoQuUL6csU3nPoVrVjkNf1Xzrtq1dYnAB7o21827CpN7GLSZPQywFmit3bYfk4SYcdf8vXf3hMcvHUNw5u44jtUoupT2i1cdbjcXvoS7c16dfK1qKxtcRy6iXqoGQ7F7KYveEqc9GcuEzEhGijV3UEFX9oA7WkuLh1JVFuXcrqMw3CBsrhApM8VKuVEwj76ofqazJtrr4aprFWemJ85h1QwrD3L1";
    let data_a = bs58::decode(data).into_vec()?;
    let transaction: VersionedTransaction = bincode::deserialize(&data_a)?;
    let result = client.send_and_confirm_transaction_with_spinner(&transaction);
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
