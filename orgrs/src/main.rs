use clap_conf::*;
mod server;
mod orgdb;


#[tokio::main]
async fn main() {
	let args = clap_app!(orgrc => 
								(version: crate_version!())
								(author: "Ian Davids")
								(about: "OrgRs Org Mode Server")
								(@arg connect: -c --connect +takes_value "Server connection")
								(@arg org: -o --orgglob +takes_value "org glob")
							).get_matches();
	let cfg = clap_conf::with_toml_env(&args, &["{HOME}/.config/orgrs/init.toml","{HOME}/.orgrs.toml","./.orgrs.toml"]);
	let connect_str = cfg.grab().arg("connect").conf("server.connect").env("ORGRS_CONNECT").def("ws://127.0.0.1:3030/orgrs");
	let org_glob = cfg.grab().arg("org").conf("org.glob").env("ORGRS_GLOB").def("**/*.org");

    let mut db = orgdb::OrgDb::new();
    db.reload_all(&org_glob).await;
    db.list_all_files().await;

    let server = server::OrgServer {};
    server.start(&connect_str);
}
