//use zksync::{provider::Provider,
//EthereumProvider, Network, RpcProvider, Wallet, WalletCredentials,};
//use zksync::{WalletCredentials};
use web3::futures::Future;


#[tokio::main]
async fn main() {


    let provider = RpcProvider::new(Network::Rinkeby);

    println!("Hello, world!");
let transport = web3::transports::Http::new("http://localhost:8545");

    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await.unwrap();
    println!("Accounts: {:?}", accounts);


 // let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
 // let web3 = web3::Web3::new(transport);
 // let accounts = web3.eth().accounts().wait().unwrap();

 // println!("Accounts: {:?}", accounts);


}
