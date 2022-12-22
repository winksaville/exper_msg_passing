use criterion::{criterion_group, criterion_main, Criterion};
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use exper_msg_passing::Message;
type Msg = Message;

//use exper_msg_passing::MsgMf;
//type Msg = MsgMf;

fn partner_echo_clone(tx: Sender<Msg>, rx: Receiver<Msg>) {
    thread::spawn(move || {
        loop {
            if let Ok(msg) = rx.recv() {
                if tx.send(msg.clone()).is_err() {
                    // Disconnected, we're done.
                    return;
                }
            } else {
                // Disconnected, we're done.
                return;
            };
        }
    });
}

#[allow(unused)]
fn crit_echo_clone(c: &mut Criterion) {
    c.bench_function("crit_echo_clone", |b| {
        let (tx, partner_rx) = channel::<Msg>();
        let (partner_tx, rx) = channel::<Msg>();

        partner_echo_clone(partner_tx, partner_rx);

        let mut msg = Msg::default();
        const COUNT: usize = 0x100000; // 1M ~47us
                                       //const COUNT: usize = 0x10000; // ~6.1us
                                       //const COUNT: usize = 0x1000; // ~3.9us
                                       //const COUNT: usize = 0x100; // ~3.7us
                                       //const COUNT: usize = 0x10; // ~3.7us
                                       //const COUNT: usize = 0x1; // ~3.7us
        for i in 0..COUNT {
            msg.v.push((i & 0xff) as u8);
        }
        //println!("\ncrit_echo_clone:    msg={}", &msg);

        // Send first message to partner
        tx.send(msg).expect("crit_echo_clone: error sending msg");

        b.iter(|| {
            let msg = rx.recv().expect("crit_echo_clone: error receiving msg");
            //println!("crit_echo_clone:    msg={}", &msg);
            tx.send(msg.clone())
                .expect("crit_echo_clone: error sending msg");
        });
    });
}

fn partner_echo(tx: Sender<Msg>, rx: Receiver<Msg>) {
    thread::spawn(move || loop {
        if let Ok(msg) = rx.recv() {
            //println!("partner_echo: msg={}", msg);
            if tx.send(msg).is_err() {
                // Disconnected, we're done.
                return;
            }
        } else {
            // Disconnected, we're done.
            return;
        };
    });
}

#[allow(unused)]
fn crit_echo(c: &mut Criterion) {
    c.bench_function("crit_echo", |b| {
        let (tx, partner_rx) = channel::<Msg>();
        let (partner_tx, rx) = channel::<Msg>();

        partner_echo(partner_tx, partner_rx);

        let mut msg = Msg::default();
        const COUNT: usize = 0x100000; // 1M ~3.6us
        for i in 0..COUNT {
            msg.v.push((i & 0xff) as u8);
        }
        //println!("\ncrit_echo:    msg={}", &msg);

        // Send first message to partner
        tx.send(msg).expect("crit_echo: error sending msg");

        b.iter(|| {
            let recv_msg = rx.recv().expect("crit_echo: error receiving msg");
            //println!("crit_echo_clone:    msg={}", &msg);
            tx.send(recv_msg).expect("crit_echo: error sending msg");
        });
    });
}

fn partner_echo_box(tx: Sender<Box<Msg>>, rx: Receiver<Box<Msg>>) {
    thread::spawn(move || loop {
        if let Ok(msg) = rx.recv() {
            //println!("partner_echo_box: msg={}", msg);
            if tx.send(msg).is_err() {
                // Disconnected, we're done.
                return;
            }
        } else {
            // Disconnected, we're done.
            return;
        };
    });
}

#[allow(unused)]
fn crit_echo_box(c: &mut Criterion) {
    c.bench_function("crit_echo_box", |b| {
        let (tx, partner_rx) = channel::<Box<Msg>>();
        let (partner_tx, rx) = channel::<Box<Msg>>();

        partner_echo_box(partner_tx, partner_rx);

        let mut msg = Box::new(Msg::default());
        const COUNT: usize = 0x100000; // 1M ~3.6us
        for i in 0..COUNT {
            msg.v.push((i & 0xff) as u8);
        }
        //println!("\ncrit_echo_box:    msg={}", &msg);

        // Send first message to partner
        tx.send(msg).expect("crit_echo_box: error sending msg");

        b.iter(|| {
            let recv_msg = rx.recv().expect("crit_echo_box: error receiving msg");
            //println!("crit_echo_box_clone:    msg={}", &msg);
            tx.send(recv_msg).expect("crit_echo_box: error sending msg");
        });
    });
}

fn partner_echo_box_clone(tx: Sender<Box<Msg>>, rx: Receiver<Box<Msg>>) {
    thread::spawn(move || {
        loop {
            if let Ok(msg) = rx.recv() {
                if tx.send(msg.clone()).is_err() {
                    // Disconnected, we're done.
                    return;
                }
            } else {
                // Disconnected, we're done.
                return;
            };
        }
    });
}

#[allow(unused)]
fn crit_echo_box_clone(c: &mut Criterion) {
    c.bench_function("crit_echo_box_clone", |b| {
        let (tx, partner_rx) = channel::<Box<Msg>>();
        let (partner_tx, rx) = channel::<Box<Msg>>();

        partner_echo_box_clone(partner_tx, partner_rx);

        let mut msg = Box::new(Msg::default());
        const COUNT: usize = 0x100000; // 1M ~47us
                                       //const COUNT: usize = 0x10000; // ~6.1us
                                       //const COUNT: usize = 0x1000; // ~3.9us
                                       //const COUNT: usize = 0x100; // ~3.7us
                                       //const COUNT: usize = 0x10; // ~3.7us
                                       //const COUNT: usize = 0x1; // ~3.7us
        for i in 0..COUNT {
            msg.v.push((i & 0xff) as u8);
        }
        //println!("\ncrit_echo_box_clone:    msg={}", &msg);

        // Send first message to partner
        tx.send(msg)
            .expect("crit_echo_box_clone: error sending msg");

        b.iter(|| {
            let msg = rx.recv().expect("crit_echo_box_clone: error receiving msg");
            //println!("crit_echo_box_clone:    msg={}", &msg);
            tx.send(msg.clone())
                .expect("crit_echo_box_clone: error sending msg");
        });
    });
}

criterion_group!(
    benches,
    crit_echo_clone,
    crit_echo,
    crit_echo_box,
    crit_echo_box_clone
);
criterion_main!(benches);
