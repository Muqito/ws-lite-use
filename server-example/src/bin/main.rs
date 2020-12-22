use futures_lite::future::block_on;
use server_example::start_server;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    if let Err(err) = block_on(start_server()) {
        println!("{}", err);
    }
}
