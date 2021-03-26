use ethane::rpc;
use test_helper::*;

fn main() {
	let mut client = ConnectorWrapper::new_from_env(None);
	match client.call(rpc::personal_list_accounts()) {
		Ok(res) => println!("result = {:?}", res), 
		Err(err) => println!("error: {:?}", err),
	}
}
