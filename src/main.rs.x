/// Shows passing msg as an owned parameter in process_msg2 allows it to be
/// passed through the channel with the Vec<u8> being NOT moving. But when you
/// use process_msg1 and pass msg as a borrow the cloning does create a copy
/// of the data and we see it in a new location, i.e. it's address changes.
///
/// $ cargo run --example zero-copy
///     Finished dev [unoptimized + debuginfo] target(s) in 0.03s
///      Running `target/debug/examples/zero-copy`
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use exper_msg_passing::Message;

use custom_logger::env_logger_init;
use rand::random;

#[allow(unused)]
fn process_msg1(tx: &Sender<Message>, msg: &Message) -> i32 {
    let mut sum = 0i32;
    for v in msg.v.iter() {
        sum += *v as i32;
    }
    // Here we MUST clone because we have a reference to the message
    tx.send(msg.clone()).expect("darn");

    log::info!("process_msg1: {}", sum);
    sum
}

#[allow(unused)]
fn process_msg2(tx: &Sender<Message>, msg: Message) -> i32 {
    let mut sum = 0i32;
    for v in msg.v.iter() {
        sum += *v as i32;
    }
    // Here we DO NOT need to clone because we own the message
    // we can send it
    tx.send(msg).expect("darn");

    log::info!("process_msg2: {}", sum);
    sum
}

fn partner(tx: Sender<Message>, rx: Receiver<Message>) {
    thread::spawn(move || loop {
        let recv_msg = match rx.recv() {
            Ok(m) => m,
            Err(why) => {
                log::info!("partner: error receiving recv_msg why='{}'", why);
                break;
            }
        };
        log::info!("partner: recv_msg={}", recv_msg);
        let sum = if random::<bool>() {
            process_msg1(&tx, &recv_msg)
        } else {
            process_msg2(&tx, recv_msg)
        };
        log::info!("partner: sum={}", sum);
    });
}

fn main() {
    env_logger_init("info");
    log::info!("main:+");

    let (tx, partner_rx) = channel::<Message>();
    let (partner_tx, rx) = channel::<Message>();

    partner(partner_tx, partner_rx);

    let mut msg = Message { v: vec![1, 2, 3] };
    log::info!("main:         msg={}", msg);
    for _ in 0..10 {
        tx.send(msg).expect("main: error sending msg");
        let recv_msg = rx.recv().expect("main: error receiving recv_msg");
        log::info!("main:    recv_msg={}", &recv_msg);
        msg = recv_msg;
    }

    log::info!("main:-");
}
