mod client;
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();
    let mut io = IoHandler::default();
    // json rpc method
    io.add_method("hello", |_| Ok(Value::String("hello".into())));
    io.add_method("author", |_| Ok(Value::String(client::author())));
    io.add_method("healthz", |_| Ok(Value::String(client::healthz())));
    // io.add_method("add", |_params: Params| {
    //     Ok(Value::String(client::add(_params)))
    // });

    info!("Start Server !!!");
    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .expect("Unable to start RPC server");
    server.wait();
}
