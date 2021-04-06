extern crate clap;
use amiquip::{Connection, Exchange, Publish, QueueDeclareOptions, Result};
use clap::{App, Arg};
use crossbeam_channel::{bounded, select, tick};
use std::time;

fn main() -> Result<()> {
    let matches = App::new("Beagle - rabbitmq message generating client")
        .version("1.0")
        .author("Maciej Ł. <maciej@lados.dev>")
        .about("Simple program for generating and sending rabbitmq messages to populate the queue")
        .arg(
            Arg::with_name("URI")
                .help("AMQP scheme uri representing host to connect to, with potential plain auth")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("MESSAGE")
                .help("String encoded message that will be pushed to the queue")
                .index(2),
        )
        .arg(
            Arg::with_name("channel")
                .short("c")
                .takes_value(true)
                .help("Channel used to publish the messages, chosen automatically by default"),
        )
        .arg(
            Arg::with_name("queue")
                .short("q")
                .multiple(true)
                .help("Queue to publish messages on, will be declared if it doesn't exist, use -qq to create a durable queue"),
        )
        .arg(
            Arg::with_name("loop")
                .short("l")
                .help("Repeat the message sending process every N miliseconds"),
        )
        .get_matches();

    let host = matches.value_of("URI").unwrap();
    let target_channel = matches.value_of("channel").unwrap_or("");
    let queue_name = matches.value_of("queue").unwrap_or("hello");
    let message = matches.value_of("MESSAGE").unwrap_or("hello");
    let mut connection = Connection::insecure_open(host)?;

    let channel = connection.open_channel(match target_channel {
        "" => None,
        _ => Some(
            target_channel
                .parse::<u16>()
                .expect("Please provide valid number as channel id"),
        ),
    })?;

    if matches.occurrences_of("queue") > 0 {
        println!("Declaring queue: {}", queue_name);
        channel.queue_declare(
            queue_name,
            QueueDeclareOptions {
                durable: match matches.occurrences_of("q") {
                    1 => false,
                    2 => true,
                    _ => true,
                },
                ..QueueDeclareOptions::default()
            },
        )?;
    }

    let exchange = Exchange::direct(&channel);

    if matches.is_present("loop") {
        let (sender, receiver) = bounded(1);
        let interval = matches
            .value_of("loop")
            .unwrap_or("2000")
            .parse::<u64>()
            .expect("Please provide correct interval in miliseconds");
        let ticks = tick(time::Duration::from_millis(interval));

        ctrlc::set_handler(move || {
            let _ = sender.send(());
        })
        .expect("Error setting Ctrl-C abort handler");

        loop {
            select! {
                recv(ticks) -> _ => {
                    exchange.publish(Publish::new(message.as_bytes(), queue_name))?;
                    println!("[{}] Published message '{}' on queue {}", chrono::Local::now(), message, queue_name)
                }
                recv(receiver) -> _ => {
                    println!("");
                    break
                 }
            }
        }
    } else {
        exchange.publish(Publish::new(message.as_bytes(), queue_name))?;
        println!("Published message '{}' on queue {}", message, queue_name)
    }

    println!("Closing message connection");
    connection.close()
}
