use jsonrpc_ws_server::*;
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;

#[rpc]
pub trait Rpc {
	/// Returns a protocol version
	#[rpc(name = "protocolVersion")]
	fn protocol_version(&self) -> Result<String>;

	/// Adds two numbers and returns a result
	#[rpc(name = "add", alias("callAsyncMetaAlias"))]
	fn add(&self, a: u64, b: u64) -> Result<u64>;

	/// Performs asynchronous operation
	#[rpc(name = "callAsync")]
	fn call(&self, a: u64) -> BoxFuture<Result<String>>;

	/// Querries for headlines that match a query
	#[rpc(name = "query_headline")]
	fn query_headline(&self, query: String) -> Result<Vec<String>>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
