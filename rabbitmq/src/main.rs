use amiquip::{Connection, Exchange, Publish, Result};
use std::{env, thread, time};

fn main() -> Result<()> {
    let host = env::args()
        .nth(1)
        .expect("Please provide queue host to connect with");
    let user = env::args().nth(2).unwrap_or_default();
    let password = env::args().nth(3).unwrap_or_default();

    // Open connection.
    let mut connection =
        Connection::insecure_open(&*format!("amqp://{}:{}@{}", user, password, host))?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    loop {
        exchange.publish(Publish::new("hello there".as_bytes(), "hello"))?;
        println!("Published hello message");
        thread::sleep(time::Duration::from_secs(3));
    }
    // Publish a message to the "hello" queue.
    connection.close()
}
