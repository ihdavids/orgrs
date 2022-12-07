use clap::Parser;
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
    let args = Args::parse();
    let server = server::OrgServer {};
    server.Start();
    println!("Hello, world! {}", args.count);
}
