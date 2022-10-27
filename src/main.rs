use std::net::Ipv4Addr;
use std::time::Duration;

use futures::{stream, stream::StreamExt, TryFutureExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::ip_iterator::IpIterator;

mod ip_iterator;

async fn process(addr: Ipv4Addr) {
    let timeout = timeout(
        Duration::from_secs(1),
        TcpStream::connect((addr, 25565))
    ).await;

    match timeout {
        Ok(Ok(mut stream)) => {
            match craftping::tokio::ping(&mut stream, "localhost", 25565).await {
                Ok(resp) => println!("{}, {}, {}", addr, resp.protocol, resp.version),
                _ => {}
            }
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() {
    stream::iter(IpIterator::new())
        .map(|addr| process(addr))
        .buffer_unordered(5000)
        .collect::<()>()
        .await;
}
