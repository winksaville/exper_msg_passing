use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration};
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use exper_msg_passing::{MsgMf, MsgOf, PushV};

const KB: usize = 1024;

fn prep<Msg: PushV + Clone + Default>(
    v_len: usize,
) -> (Sender<Msg>, Sender<Msg>, Receiver<Msg>, Receiver<Msg>) {
    let (tx, partner_rx) = channel::<Msg>();
    let (partner_tx, rx) = channel::<Msg>();

    let mut msg = Msg::default();
    for i in 0..v_len {
        msg.push_v((i & 0xff) as u8);
    }

    tx.send(msg).expect("prep: error sending msg");

    (tx, partner_tx, rx, partner_rx)
}

fn partner<Msg: std::marker::Send + 'static>(
    tx: Sender<Msg>,
    rx: Receiver<Msg>,
) {
    thread::spawn(move || {
        loop {
            if let Ok(msg) = rx.recv() {
                if tx.send(msg).is_err() {
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

fn partner_clone<Msg: Clone + std::marker::Send + 'static>(
    tx: Sender<Msg>,
    rx: Receiver<Msg>,
) {
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

fn prep_box<Msg: PushV + Clone + Default>(
    v_len: usize,
) -> (Sender<Box<Msg>>, Sender<Box<Msg>>, Receiver<Box<Msg>>, Receiver<Box<Msg>>) {
    let (tx, partner_rx) = channel::<Box<Msg>>();
    let (partner_tx, rx) = channel::<Box<Msg>>();

    let mut msg = Msg::default();
    for i in 0..v_len {
        msg.push_v((i & 0xff) as u8);
    }

    tx.send(Box::new(msg)).expect("crit_echo_clone: error sending msg");

    (tx, partner_tx, rx, partner_rx)
}

fn partner_box<Msg: std::marker::Send + 'static>(
    tx: Sender<Box<Msg>>,
    rx: Receiver<Box<Msg>>,
) {
    thread::spawn(move || {
        loop {
            if let Ok(msg) = rx.recv() {
                if tx.send(msg).is_err() {
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

fn partner_clone_box<Msg: Clone + std::marker::Send + 'static>(
    tx: Sender<Box<Msg>>,
    rx: Receiver<Box<Msg>>,
) {
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
fn echo(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("echo");
    group.plot_config(plot_config);

    for v_len in [
        0x1,
        4 * KB,
        64 * KB,
        128 * KB,
        256 * KB,
        512 * KB,
        1024 * KB,
    ] {
        group.bench_with_input(BenchmarkId::new("MsgOf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner) = prep::<MsgOf>(v_len);

            partner::<MsgOf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("echo MsgOf: error receiving msg");
                //println!("echo MsgOf:    msg={}", &msg);
                tx.send(msg)
                    .expect("echo MsgOf: error sending msg");
            });
        });
        group.bench_with_input(BenchmarkId::new("box MsgOf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner): (Sender<Box<MsgOf>>, Sender<Box<MsgOf>>, Receiver<Box<MsgOf>>, Receiver<Box<MsgOf>>) = prep_box::<MsgOf>(v_len);

            partner_box::<MsgOf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("echo box MsgOf: error receiving msg");
                //println!("echo MsgOf:    msg={}", &msg);
                tx.send(msg)
                    .expect("echo box MsgOf: error sending msg");
            });
        });
        group.bench_with_input(BenchmarkId::new("MsgMf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner) = prep::<MsgMf>(v_len);

            partner::<MsgMf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("echo MsgMf: error receiving msg");
                //println!("echo MsgMf:    msg={}", &msg);
                tx.send(msg)
                    .expect("echo MsgMf: error sending msg");
            });
        });
        group.bench_with_input(BenchmarkId::new("box MsgMf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner) = prep_box::<MsgMf>(v_len);

            partner_box::<MsgMf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("echo box MsgMf: error receiving msg");
                //println!("echo MsgMf:    msg={}", &msg);
                tx.send(msg)
                    .expect("echo box MsgMf: error sending msg");
            });
        });
    }
}

#[allow(unused)]
fn echo_clone(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("echo_clone");
    group.plot_config(plot_config);

    for v_len in [
        0x1,
        4 * KB,
        64 * KB,
        128 * KB,
        256 * KB,
        512 * KB,
        1024 * KB,
    ] {
        group.bench_with_input(BenchmarkId::new("clone MsgOf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner) = prep::<MsgOf>(v_len);

            partner_clone::<MsgOf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("clone MsgOf: error receiving msg");
                //println!("echo clone MsgOf:    msg={}", &msg);
                tx.send(msg.clone())
                    .expect("clone MsgOf: error sending msg");
            });
        });
        group.bench_with_input(BenchmarkId::new("clone box MsgOf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner): (Sender<Box<MsgOf>>, Sender<Box<MsgOf>>, Receiver<Box<MsgOf>>, Receiver<Box<MsgOf>>) = prep_box::<MsgOf>(v_len);

            partner_clone_box::<MsgOf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("echo clone box MsgOf: error receiving msg");
                //println!("echo MsgOf:    msg={}", &msg);
                tx.send(msg.clone())
                    .expect("clone box MsgOf: error sending msg");
            });
        });
        group.bench_with_input(BenchmarkId::new("clone MsgMf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner) = prep::<MsgMf>(v_len);

            partner_clone::<MsgMf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("echo clone MsgMf: error receiving msg");
                //println!("echo clone MsgMf:    msg={}", &msg);
                tx.send(msg.clone())
                    .expect("clone MsgMf: error sending msg");
            });
        });
        group.bench_with_input(BenchmarkId::new("clone box MsgMf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner) = prep_box::<MsgMf>(v_len);

            partner_clone_box::<MsgMf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("echo clone box MsgMf: error receiving msg");
                //println!("echo MsgMf:    msg={}", &msg);
                tx.send(msg.clone())
                    .expect("clone box MsgMf: error sending msg");
            });
        });
    }
}


criterion_group!(
    benches,
    echo,
    echo_clone,
);
criterion_main!(benches);
