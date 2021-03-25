use ethane::rpc;
use test_helper::*;

fn main() {
	let mut client = ConnectorWrapper::new_from_env(None);
	match client.call(rpc::eth_block_number()) {
		Ok(res) => println!("result = {:?}", res), 
		Err(err) => println!("error: {:?}", err),
	}
}
