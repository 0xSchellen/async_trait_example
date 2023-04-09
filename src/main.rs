#![feature(async_fn_in_trait)]
// warning: the feature `async_fn_in_trait` is incomplete and may not be safe to use and/or cause compiler crashes
//  --> src/main.rs:1:12
//   |
// 1 | #![feature(async_fn_in_trait)]
//   |            ^^^^^^^^^^^^^^^^^
//   |
//   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
//   = note: `#[warn(incomplete_features)]` on by default

mod provider;
use provider::*;
pub use primitive_types::U256;

#[tokio::main]
async fn main() {
    let provider: Provider = Provider::new("https://eth.llamarpc.com");

    let result = provider.get_block_by_number(Some(DefaultBlockParam::PENDING), None).await.unwrap().unwrap();

    println!("============================");
    println!("result: {result:#?}");

    println!("============================");
    let number = result.number.unwrap();
    println!("number     u256 : {number:#?}");  

    println!("============================");
    let gas_limit = result.gas_limit;
    println!("gas_limit  u256 : {gas_limit:#?}");

    println!("============================");
}
