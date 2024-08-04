## solana-payments
<img src="https://raw.githubusercontent.com/448-engineering/SolanaPayments/master/Brand-Collateral/solana-payments-icon.svg" alt="Solana Payments Logo" width=50%>

A lightweight library for parsing and creating Solana Pay URLs written in Rust.

#### Parsing a URL with an SPL Token and a lookup function
```rust,no_run
#[tokio::main]
async fn main() {
use solana_payments::SolanaPayUrl;
use solana_client::nonblocking::rpc_client::RpcClient;
use spl_token_2022::{state::Mint, extension::StateWithExtensions};

let transfer_1_sol = "solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=1&label=Michael&message=Thanks%20for%20all%20the%20fish&memo=OrderId12345";

// parsing requires a lookup function to be passed in that fetches the account
// data of the spl-token token mint account which the deserializes it's data to get 
// the number of decimals configured for the Mint
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
```

Which returns the following output
```sh
SolanaPayUrl {
    recipient: PublicKey(mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN),
    amount: Some(
        Number {
            integral: 0,
            fractional: 1,
            leading_zeroes: 1,
            significant_digits_count: 1,
            as_string: "0.01",
            total_fractional_count: 2,
        },
    ),
    spl_token: Some(
        PublicKey(EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v),
    ),
    references: [],
    label: None,
    message: None,
    spl_memo: None,
}
```
#### Parsing a URL for native SOL with a lookup function
```rust
use solana_payments::SolanaPayUrl;

let transfer_1_sol = "solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=1&label=Michael&message=Thanks%20for%20all%20the%20fish&memo=OrderId12345";

// Return native SOL decimals
let lookup_fn = |_value| async { 9 };
let transfer_1_sol_decoded = smol::block_on(async {
    solana_payments::SolanaPayUrl::new()
        .parse(transfer_1_sol, lookup_fn)
        .await
        .unwrap()
});

// Return native SOL decimals from the [solana_payments::Utils](crate::Utils::native_sol)
// helper if you don't want to send a request to an RPC to lookup native SOL decimals
let transfer_1_sol_decoded = smol::block_on(async {
    SolanaPayUrl::new()
        .parse(transfer_1_sol, solana_payments::Utils::native_sol)
        .await
        .unwrap()
});
```

#### Construct a Solana Pay URL from scratch
```rust
use solana_payments::SolanaPayUrl;

let url_encoded_str = SolanaPayUrl::default()
    .add_recipient("mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN")
    .unwrap()
    .add_amount("1")
    .unwrap()
    .add_spl_token("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
    .unwrap()
    .add_label("Michael")
    .unwrap()
    .add_message("Thanks for all the fish")
    .unwrap()
    .add_spl_memo("OrderId12345")
    .unwrap()
    .add_reference("7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNatx")
    .unwrap()
    .add_reference_multiple(&[
        "7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNatx",
        "7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNatx",
        "7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNatx",
    ])
    .unwrap()
    .to_url();

```



### LICENSE
This work is released into the public domain under [CC0-1.0](https://choosealicense.com/licenses/cc0-1.0/#) LICENSE alternatively it is licensed under [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)