use clap::Parser;

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
    println!("Hello, world! {}", args.name);
}
