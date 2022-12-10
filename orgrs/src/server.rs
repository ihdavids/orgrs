use jsonrpc_ws_server::*;
use jsonrpc_core::futures::future::{self};
use jsonrpc_core::{BoxFuture, IoHandler, Result};
use orgcom::Rpc;


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
    pub fn start(&self)
    {
        let mut io = IoHandler::new();
		io.extend_with(RpcImpl.to_delegate());
   
		println!("STARTING SERVER");
        let server = ServerBuilder::new(io)
            .start(&"0.0.0.0:3030".parse().unwrap())
            .expect("Server must start with no issues");
    
		println!("RUNNING WAIT ON SERVER");
        let v = server.wait().unwrap();
		println!("WAIT UNRAPPED");
		return v;
    }
}
