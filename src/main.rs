/**
 * @Author: @iamrezamousavi
 * @Date:   2023-03-24 02:17:10
 * @Last Modified by:   @iamrezamousavi
 * @Last Modified time: 2023-03-26 21:23:50
 */
use rust_todo::server::TodoServer;
use structopt::StructOpt;

/// A minimul RESTful api
#[derive(StructOpt, Debug)]
#[structopt(name = "rust-todo")]
struct Opt {
    /// Set host
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: String,

    /// Set port
    #[structopt(short, long, default_value = "8080")]
    port: u16,
}

fn main() {
    let opt = Opt::from_args();
    let server = TodoServer::new(opt.host, opt.port);
    server.run().unwrap();
}
