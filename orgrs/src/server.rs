use jsonrpc_ws_server::*;
use jsonrpc_core::futures::future::{self};
use jsonrpc_core::{BoxFuture, IoHandler, Result};
use std::net::SocketAddr;
use orgcom::Rpc;
use log::{info};
use crate::orgdb::OrgDb;
use fasteval;

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
		let hold = OrgDb::get();
        let ydb = hold.lock().expect("Failed to access OrgDb in Query Headline");

		/* 
	    // This example doesn't use any variables, so just use an EmptyNamespace:
 	    let mut ns = fasteval::EmptyNamespace;
    	let parser = fasteval::Parser::new();
    	let mut slab = fasteval::Slab::new();
    	//let mut map = BTreeMap::new();
	    let mut cb = |name:&str, args:Vec<f64>| -> Option<f64> {
 	    	let mydata : [f64; 3] = [11.1, 22.2, 33.3];
  	    	match name {
            // Custom constants/variables:
            "x" => Some(3.0),
            "y" => Some(4.0),

            // Custom function:
            "sum" => Some(args.into_iter().sum()),

            // Custom array-like objects:
            // The `args.get...` code is the same as:
            //     mydata[args[0] as usize]
            // ...but it won't panic if either index is out-of-bounds.
            "data" => args.get(0).and_then(|f| mydata.get(*f as usize).copied()),

            // A wildcard to handle all undefined names:
            _ => None,
     	   }
    	};


    	let expr_str = "sin(deg/360 * 2*pi())";
    	let compiled = parser.parse(expr_str, &mut slab.ps)?.from(&slab.ps).compile(&slab.ps, &mut slab.cs);
    	for deg in 0..360 {
        	//map.insert("deg".to_string(), deg as f64);
        	// When working with compiled constant expressions, you can use the
        	// eval_compiled*!() macros to save a function call:
        	let val = fasteval::eval_compiled!(compiled, &slab, &mut cb);
        	//eprintln!("sin({}°) = {}", deg, val);
    	}
*/
        for (name, _) in &ydb.by_file {
            println!("-> {name}");
        }
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
