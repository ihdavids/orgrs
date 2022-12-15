use jsonrpc_ws_server::*;
use jsonrpc_core::futures::future::{self};
use jsonrpc_core::{BoxFuture, IoHandler, Result, ErrorCode};
use std::net::SocketAddr;
use orgcom::Rpc;
use log::{info};
use crate::orgdb::OrgDb;
use fasteval;
use fasteval::{Evaler,Compiler,Instruction,Slab};
use orgize::{Org};

struct RpcImpl;


fn eval_node(node: &Org, name: &String, compiled: &Instruction, slab: &Slab) -> std::result::Result<bool,fasteval::Error> {
		// Make this callback handle each node properly
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

        let val = compiled.eval(slab, &mut cb)?;
		if val >= 0.0 && val <= 0.0 {
			Ok(false)
		} else {
			Ok(true)
		}
}


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

		 
	    // This example doesn't use any variables, so just use an EmptyNamespace:
 	    let mut ns = fasteval::EmptyNamespace;
    	let parser = fasteval::Parser::new();
    	let mut slab = fasteval::Slab::new();
    	//let mut map = BTreeMap::new();

    	//let expr_str = "sin(deg/360 * 2*pi())";
    	let exprRes = parser.parse(query.as_str(), &mut slab.ps);
		let compiled = match exprRes {
			Ok(res) => {
				res.from(&slab.ps).compile(&slab.ps, &mut slab.cs)
			},
			Err(_) => {
				return Err(jsonrpc_core::Error {code: ErrorCode::InvalidParams, message: String::from("Failed to parse expression"), data: None});
			}
		};	
        for (name, node) in &ydb.by_file {
			// TODO: Iterate over "nodes" and extract info.
			// TODO: Handle that expect
			for (item) in node.iter() {
				if(eval_node(node, name, &compiled, &slab).expect("Hi")) {
				
				}
			}
            //println!("-> {name}");
        }

    	for deg in 0..360 {
        	//map.insert("deg".to_string(), deg as f64);
        	// When working with compiled constant expressions, you can use the
        	// eval_compiled*!() macros to save a function call:
        	//eprintln!("sin({}°) = {}", deg, val);
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
