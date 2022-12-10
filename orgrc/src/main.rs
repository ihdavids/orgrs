use clap::Parser;
//use jsonrpc_core::futures::future::Future;
use jsonrpc_core_client::transports::ws;
use serde_json::json;
//use serde_json::*;
use url::Url;
use jsonrpc_core::futures::FutureExt;

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


#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("Hello, world! {}", args.name);


	let client_url = Url::parse("ws://127.0.0.1:3030").unwrap();
	let client = ws::connect::<orgcom::gen_client::Client>(&client_url).await.unwrap();

	let mut interval = serde_json::map::Map::new();
	interval.insert("interval".to_string(), 1000.into());

	client
           .clone()
    	   .add(42,42)
           .map(|res| println!("add = {:?}", res))
           .await
           ;
	   //.unwrap();
}
