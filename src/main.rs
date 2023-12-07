mod client;
mod util;
mod world;

fn main() {
    pollster::block_on(client::run_client());
}
