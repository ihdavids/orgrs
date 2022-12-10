//use clap::Parser;
//use jsonrpc_core::futures::future::Future;
use jsonrpc_core_client::transports::ws;
//use serde_json::json;
//use serde_json::*;
use url::Url;
use jsonrpc_core::futures::FutureExt;
use clap_conf::*;

#[tokio::main]
async fn main() {
	let args = clap_app!(orgrc => 
								(version: crate_version!())
								(author: "Ian Davids")
								(about: "OrgRs command line utility")
								(@arg connect: -c "Server connection")
							).get_matches();
	let cfg = clap_conf::with_toml_env(&args, &["{HOME}/.config/orgrc/init.toml","{HOME}/.orgrc.toml","./.orgrc.toml"]);
	let connect_str = cfg.grab().arg("connect").conf("server.connect").env("ORGRC_CONNECT").def("ws://127.0.0.1:3030/orgrs");
    println!("Hello, world! {}", connect_str);


	let client_url = Url::parse(&connect_str).unwrap();
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
