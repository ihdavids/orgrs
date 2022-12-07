use jsonrpc_ws_server::*;
use jsonrpc_ws_server::jsonrpc_core::*;

//use jsonrpc_core;
//use ws;
//use self::server_utils::tokio;


pub struct OrgServer
{

}

impl OrgServer 
{
    pub fn start(&self)
    {
        /* 
        let mut io = IoHandler::default();
        io.add_method("say_hello", |_params: Params| async {
            Ok(Value::String("hello".to_owned()))
        });
    
        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&"127.0.0.1:3030".parse().unwrap())
            .unwrap();
    
        server.wait();
*/
        let mut io = IoHandler::new();
        io.add_method("say_hello", |_| async {
            Ok(Value::String("Hello World!".into()))
        });
    
        let server = ServerBuilder::new(io)
            .start(&"0.0.0.0:3030".parse().unwrap())
            .expect("Server must start with no issues");
    
        server.wait().unwrap()    
    }
}
