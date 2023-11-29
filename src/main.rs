mod client;
mod util;

fn main() {
    pollster::block_on(client::run_client());
}
