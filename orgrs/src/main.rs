use clap::Parser;
use clap_conf::*;
mod server;


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


fn main() {
	let args = clap_app!(orgrc => 
								(version: crate_version!())
								(author: "Ian Davids")
								(about: "OrgRs Org Mode Server")
								(@arg connect: -c "Server connection")
							).get_matches();
	let cfg = clap_conf::with_toml_env(&args, &["{HOME}/.config/orgrs/init.toml","{HOME}/.orgrs.toml","./.orgrs.toml"]);
	let connect_str = cfg.grab().arg("connect").conf("server.connect").env("ORGRS_CONNECT").def("ws://127.0.0.1:3030/orgrs");

    let server = server::OrgServer {};
    server.start(&connect_str);
}
