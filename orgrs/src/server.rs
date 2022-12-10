use jsonrpc_ws_server::*;
//use jsonrpc_ws_server::jsonrpc_core::Value;

//use jsonrpc_core::Value;
//use ws;
//use self::server_utils::tokio;

//use jsonrpc_core_client::transports::local;
use jsonrpc_core::futures::future::{self};
//use jsonrpc_core::futures::{self, Future};
//use jsonrpc_core::{self, FutureResult};
use jsonrpc_core::{BoxFuture, IoHandler, Result};
//use jsonrpc_derive::rpc;
//use futures_util::future::*;
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
		//future::ok("OK".to_owned())
	}
}
/* 
fn main() {
	let mut io = IoHandler::new();
	io.extend_with(RpcImpl.to_delegate());

	let fut = {
		let (client, server) = local::connect::<gen_client::Client, _, _>(io);
		client.add(5, 6).map(|res| println!("5 + 6 = {}", res)).join(server)
	};
	fut.wait().unwrap();
}
*/

pub struct OrgServer
{

}

impl OrgServer 
{
    pub fn start(&self)
    {
        /* 
        let mut io = IoHandler::default();
        io.add_method("say_hello", |_params: Params| async {
            Ok(Value::String("hello".to_owned()))
        });
    
        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&"127.0.0.1:3030".parse().unwrap())
            .unwrap();
    
        server.wait();
*/
        let mut io = IoHandler::new();
        //io.add_method("say_hello", |_| async {
        //    Ok(jsonrpc_ws_server::jsonrpc_core::Value::String("Hello World!".into()))
        //});
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
