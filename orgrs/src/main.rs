use clap_conf::*;
mod server;
mod orgdb;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    trace!("OrgRs Starting Up!");
    pretty_env_logger::init_custom_env("ORGRS_LOG");
	let args = clap_app!(orgrc => 
								(version: crate_version!())
								(author: "Ian Davids")
								(about: "OrgRs Org Mode Server")
								(@arg connect: -c --connect +takes_value "Server connection")
								(@arg org: -o --orgdir +takes_value "org directory")
							).get_matches();
	let cfg = clap_conf::with_toml_env(&args, &["{HOME}/.config/orgrs/init.toml","{HOME}/.orgrs.toml","./.orgrs.toml"]);
	let connect_str = cfg.grab().arg("connect").conf("server.connect").env("ORGRS_CONNECT").def("ws://127.0.0.1:3030/orgrs");

	let org_dir = cfg.grab().arg("org").conf("org.dir").env("ORGRS_DIR").def("./");
    let org_glob = org_dir.clone() + "**/*.org";

    trace!("OrgDb Creation!");
    {
        let db = orgdb::OrgDb::get();
        db.lock().unwrap().reload_all(&org_glob).await;
        db.lock().unwrap().list_all_files().await;
    }
    orgdb::OrgDb::watch(&org_dir).expect("Failed to setup watch on org directory ABORT!");
    
    let server = server::OrgServer {};
    server.start(&connect_str);
}
