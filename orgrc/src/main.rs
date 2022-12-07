use clap::Parser;
use jsonrpc_core::futures::future::Future;
use jsonrpc_core_client::transports::ws;
use jsonrpc_derive::rpc;
use serde_json::json;
use tokio::runtime::Runtime;
use url::Url;

/// Org Mode Server - provides websocket access to org files.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

/// Rpc trait
#[rpc(client)]
pub trait Rpc {
	/// Returns a protocol version
	#[rpc(name = "protocolVersion")]
	fn protocol_version(&self) -> Result<String>;

	/// Adds two numbers and returns a result
	#[rpc(name = "add", alias("callAsyncMetaAlias"))]
	fn add(&self, a: u64, b: u64) -> Result<u64>;

	/// Ping server expect return in interval ms
	#[rpc(name = "ping", raw_params)]
	fn ping(&self, params: Value) -> Result<String>;

	/// Performs asynchronous operation
	#[rpc(name = "callAsync")]
	fn call(&self, a: u64) -> FutureResult<String, Error>;
}


fn main() {
    let args = Args::parse();
    println!("Hello, world! {}", args.name);

    let mut rt = Runtime::new().unwrap();

	let client_url = Url::parse("ws://127.0.0.1:8888/kurento").unwrap();
	let client = rt.block_on(ws::connect::<gen_client::Client>(&client_url)).unwrap();

	let mut interval = serde_json::map::Map::new();
	map.insert("interval".to_string(), 1000.into());

	client
           .clone()
           .ping(json!({"interval": 1000}).into())
           .map(|res| println!("ping = {}", res))
           .wait()
	   .unwrap();

	rt.shutdown_now().wait().unwrap();


}



/// Rpc trait
#[rpc(client)]
pub trait Rpc {
	/// Returns a protocol version
	#[rpc(name = "protocolVersion")]
	fn protocol_version(&self) -> Result<String>;

	/// Adds two numbers and returns a result
	#[rpc(name = "add", alias("callAsyncMetaAlias"))]
	fn add(&self, a: u64, b: u64) -> Result<u64>;

	/// Ping server expect return in interval ms
	#[rpc(name = "ping", raw_params)]
	fn ping(&self, params: Value) -> Result<String>;

	/// Performs asynchronous operation
	#[rpc(name = "callAsync")]
	fn call(&self, a: u64) -> FutureResult<String, Error>;
}
