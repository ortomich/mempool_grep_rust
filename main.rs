use log::{error, warn};
//use log::info;

use web3::futures::{TryStreamExt};
use web3::transports::{WebSocket};
use web3::types::{TransactionId};

#[tokio::main]
async fn main() -> web3::Result {
    env_logger::init();

    let wss_node_endpoint = "wss://eth-mainnet.alchemyapi.io/v2/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    let sub_transport = WebSocket::new(wss_node_endpoint).await?;
    let web3 = web3::Web3::new(sub_transport);

    let mut pending_transactions = web3.eth_subscribe().subscribe_new_pending_transactions().await?;

    while let Some(pending_transaction_hash) = pending_transactions.try_next().await? {
        let pth = TransactionId::from(pending_transaction_hash);

        let res = web3.eth().transaction(pth).await;
        //println!("{:?}", res);
        match res {
            Ok(opt_txn) => {
                match opt_txn {
                    None => { warn!("could not find transaction for now") },
                    Some(txn) => println!("{:?}", txn)
                }
            }
            Err(e) => error!("{:?}", e)
        }
    }

    Ok(())
}