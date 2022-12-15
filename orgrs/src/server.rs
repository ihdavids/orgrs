use jsonrpc_ws_server::*;
use jsonrpc_core::futures::future::{self};
use jsonrpc_core::{BoxFuture, IoHandler, Result};
use std::net::SocketAddr;
use orgcom::Rpc;
use log::{info};

struct RpcImpl;

impl Rpc for RpcImpl {
	fn protocol_version(&self) -> Result<String> {
		Ok("version1".into())
	}

	fn add(&self, a: u64, b: u64) -> Result<u64> {
		println!("ADDING A{} and B{}",a,b);
		Ok(a + b)
	}

	fn call(&self, _: u64) -> BoxFuture<Result<String>> {
        Box::pin(future::ready(Ok("OK".to_owned())))
	}

	fn query_headline(&self, query: String) -> Result<Vec<String>> {
		let r:Vec<String> = Vec::new();
		return Ok(r)
	}
}

pub struct OrgServer
{

}

impl OrgServer 
{
    pub fn start(&self, connect_str: &String)
    {
        let mut io = IoHandler::new();
		io.extend_with(RpcImpl.to_delegate());
   
		info!("STARTING SERVER");
		let sock_addr: SocketAddr = connect_str.parse().expect("Unable to parse socket address");
        let server = ServerBuilder::new(io)
            .start(&sock_addr)
            .expect("Server must start with no issues");
    
		info!("RUNNING WAIT ON SERVER");
        let v = server.wait().unwrap();
		info!("WAIT UNRAPPED");
		return v;
    }
}
