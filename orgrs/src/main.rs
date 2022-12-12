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
								(@arg org: -o --orgdir +takes_value "org directory")
							).get_matches();
	let cfg = clap_conf::with_toml_env(&args, &["{HOME}/.config/orgrs/init.toml","{HOME}/.orgrs.toml","./.orgrs.toml"]);
	let connect_str = cfg.grab().arg("connect").conf("server.connect").env("ORGRS_CONNECT").def("ws://127.0.0.1:3030/orgrs");

	let org_dir = cfg.grab().arg("org").conf("org.dir").env("ORGRS_DIR").def("./");
    let org_glob = org_dir.clone() + "**/*.org";

    let mut db = orgdb::OrgDb::new();
    db.lock().unwrap().reload_all(&org_glob).await;
    db.lock().unwrap().list_all_files().await;
    orgdb::OrgDb::watch(db.clone(), &org_dir);
    //db.reload_all(&org_glob).await;
    //db.list_all_files().await;
    //db.watch(&org_dir).await.unwrap();
    
    let server = server::OrgServer {};
    server.start(&connect_str);
}
