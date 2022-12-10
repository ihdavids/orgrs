use jsonrpc_ws_server::*;
use jsonrpc_core::futures::future::{self};
use jsonrpc_core::{BoxFuture, IoHandler, Result};
use orgcom::Rpc;
use std::net::SocketAddr;


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
   
		println!("STARTING SERVER");
		let sock_addr: SocketAddr = connect_str.parse().expect("Unable to parse socket address");
        let server = ServerBuilder::new(io)
            .start(&sock_addr)
            .expect("Server must start with no issues");
    
		println!("RUNNING WAIT ON SERVER");
        let v = server.wait().unwrap();
		println!("WAIT UNRAPPED");
		return v;
    }
}
