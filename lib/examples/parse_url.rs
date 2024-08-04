use solana_client::nonblocking::rpc_client::RpcClient;
use solana_payments::SolanaPayUrl;
use spl_token_2022::{extension::StateWithExtensions, state::Mint};

#[tokio::main]
async fn main() {
    let lookup_fn = |public_key: [u8; 32]| async move {
        let client = RpcClient::new("https://api.mainnet-beta.solana.com".into());

        let public_key = solana_program::pubkey::Pubkey::new_from_array(public_key);
        let account = client.get_account(&public_key).await.unwrap();

        let mint = StateWithExtensions::<Mint>::unpack(&account.data).unwrap();
        mint.base.decimals
    };

    let url  = "solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=0.01&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    let url_decoded = SolanaPayUrl::new().parse(&url, lookup_fn).await.unwrap();

    dbg!(url_decoded);
}
