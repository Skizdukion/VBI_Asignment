use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait ExampleRPC {
    #[rpc(name = "get_value")]
    fn get_value(&self) -> Result<u32>;
}

pub struct Example;

impl ExampleRPC for Example {
    fn get_value(&self) -> Result<u32> {
        Ok(5)
    }
}