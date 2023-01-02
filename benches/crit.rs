use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration};
use exper_msg_passing::{
    Client, Echo, MainMsgs, Pinger, Server, ServiceManager, StartMsg, SuperProtocol,
};
use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};
use std::fmt::Display;

pub trait PushV {
    fn push_v(&mut self, val: u8);
}

#[derive(Clone, Default)]
pub struct MsgOf {
    pub v: Vec<u8>,
}

impl Display for MsgOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p} {:p}", &self, &self.v[0])
    }
}

impl PushV for MsgOf {
    fn push_v(&mut self, val: u8) {
        self.v.push(val);
    }
}

// Message with many fields
#[derive(Clone)]
pub struct MsgMf {
    pub v: Vec<u8>,
    pub f0u128: u128,
    pub f0u64: u64,
    pub f0u32: u32,
    pub f0u16: u16,
    pub f0u8: u8,
    pub f0i128: i128,
    pub f0i64: i64,
    pub f0i32: i32,
    pub f0i16: i16,
    pub f0i8: i8,
    pub f1u128: u128,
    pub f1u64: u64,
    pub f1u32: u32,
    pub f1u16: u16,
    pub f1u8: u8,
    pub f1i128: i128,
    pub f1i64: i64,
    pub f1i32: i32,
    pub f1i16: i16,
    pub f1i8: i8,
    pub f2u128: u128,
    pub f2u64: u64,
    pub f2u32: u32,
    pub f2u16: u16,
    pub f2u8: u8,
    pub f2i128: i128,
    pub f2i64: i64,
    pub f2i32: i32,
    pub f2i16: i16,
    pub f2i8: i8,
    pub f3u128: u128,
    pub f3u64: u64,
    pub f3u32: u32,
    pub f3u16: u16,
    pub f3u8: u8,
    pub f3i128: i128,
    pub f3i64: i64,
    pub f3i32: i32,
    pub f3i16: i16,
    pub f3i8: i8,
    pub f4u128: u128,
    pub f4u64: u64,
    pub f4u32: u32,
    pub f4u16: u16,
    pub f4u8: u8,
    pub f4i128: i128,
    pub f4i64: i64,
    pub f4i32: i32,
    pub f4i16: i16,
    pub f4i8: i8,
    pub f5u128: u128,
    pub f5u64: u64,
    pub f5u32: u32,
    pub f5u16: u16,
    pub f5u8: u8,
    pub f5i128: i128,
    pub f5i64: i64,
    pub f5i32: i32,
    pub f5i16: i16,
    pub f5i8: i8,
    pub f6u128: u128,
    pub f6u64: u64,
    pub f6u32: u32,
    pub f6u16: u16,
    pub f6u8: u8,
    pub f6i128: i128,
    pub f6i64: i64,
    pub f6i32: i32,
    pub f6i16: i16,
    pub f6i8: i8,
    pub f7u128: u128,
    pub f7u64: u64,
    pub f7u32: u32,
    pub f7u16: u16,
    pub f7u8: u8,
    pub f7i128: i128,
    pub f7i64: i64,
    pub f7i32: i32,
    pub f7i16: i16,
    pub f7i8: i8,
    pub f8u128: u128,
    pub f8u64: u64,
    pub f8u32: u32,
    pub f8u16: u16,
    pub f8u8: u8,
    pub f8i128: i128,
    pub f8i64: i64,
    pub f8i32: i32,
    pub f8i16: i16,
    pub f8i8: i8,
    pub f9u128: u128,
    pub f9u64: u64,
    pub f9u32: u32,
    pub f9u16: u16,
    pub f9u8: u8,
    pub f9i128: i128,
    pub f9i64: i64,
    pub f9i32: i32,
    pub f9i16: i16,
    pub f9i8: i8,
}

impl Display for MsgMf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p} {:p}", &self, &self.v[0])
    }
}

impl PushV for MsgMf {
    fn push_v(&mut self, val: u8) {
        self.v.push(val);
    }
}

impl Default for MsgMf {
    #[inline(never)]
    fn default() -> Self {
        Self {
            v: vec![],
            f0u128: 128,
            f0u64: 64,
            f0u32: 32,
            f0u16: 16,
            f0u8: 8,
            f0i128: 128,
            f0i64: 64,
            f0i32: 32,
            f0i16: 16,
            f0i8: 8,
            f1u128: 128,
            f1u64: 64,
            f1u32: 32,
            f1u16: 16,
            f1u8: 8,
            f1i128: 128,
            f1i64: 64,
            f1i32: 32,
            f1i16: 16,
            f1i8: 8,
            f2u128: 128,
            f2u64: 64,
            f2u32: 32,
            f2u16: 16,
            f2u8: 8,
            f2i128: 128,
            f2i64: 64,
            f2i32: 32,
            f2i16: 16,
            f2i8: 8,
            f3u128: 128,
            f3u64: 64,
            f3u32: 32,
            f3u16: 16,
            f3u8: 8,
            f3i128: 128,
            f3i64: 64,
            f3i32: 32,
            f3i16: 16,
            f3i8: 8,
            f4u128: 128,
            f4u64: 64,
            f4u32: 32,
            f4u16: 16,
            f4u8: 8,
            f4i128: 128,
            f4i64: 64,
            f4i32: 32,
            f4i16: 16,
            f4i8: 8,
            f5u128: 128,
            f5u64: 64,
            f5u32: 32,
            f5u16: 16,
            f5u8: 8,
            f5i128: 128,
            f5i64: 64,
            f5i32: 32,
            f5i16: 16,
            f5i8: 8,
            f6u128: 128,
            f6u64: 64,
            f6u32: 32,
            f6u16: 16,
            f6u8: 8,
            f6i128: 128,
            f6i64: 64,
            f6i32: 32,
            f6i16: 16,
            f6i8: 8,
            f7u128: 128,
            f7u64: 64,
            f7u32: 32,
            f7u16: 16,
            f7u8: 8,
            f7i128: 128,
            f7i64: 64,
            f7i32: 32,
            f7i16: 16,
            f7i8: 8,
            f8u128: 128,
            f8u64: 64,
            f8u32: 32,
            f8u16: 16,
            f8u8: 8,
            f8i128: 128,
            f8i64: 64,
            f8i32: 32,
            f8i16: 16,
            f8i8: 8,
            f9u128: 128,
            f9u64: 64,
            f9u32: 32,
            f9u16: 16,
            f9u8: 8,
            f9i128: 128,
            f9i64: 64,
            f9i32: 32,
            f9i16: 16,
            f9i8: 8,
        }
    }
}

const KB: usize = 1024;

fn prep<Msg: PushV + Clone + Default>(
    v_len: usize,
) -> (Sender<Msg>, Sender<Msg>, Receiver<Msg>, Receiver<Msg>) {
    let (tx, partner_rx) = unbounded::<Msg>();
    let (partner_tx, rx) = unbounded::<Msg>();

    let mut msg = Msg::default();
    for i in 0..v_len {
        msg.push_v((i & 0xff) as u8);
    }

    tx.send(msg).expect("prep: error sending msg");

    (tx, partner_tx, rx, partner_rx)
}

fn partner<Msg: std::marker::Send + 'static>(tx: Sender<Msg>, rx: Receiver<Msg>) {
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

fn partner_clone<Msg: Clone + std::marker::Send + 'static>(tx: Sender<Msg>, rx: Receiver<Msg>) {
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
) -> (
    Sender<Box<Msg>>,
    Sender<Box<Msg>>,
    Receiver<Box<Msg>>,
    Receiver<Box<Msg>>,
) {
    let (tx, partner_rx) = unbounded::<Box<Msg>>();
    let (partner_tx, rx) = unbounded::<Box<Msg>>();

    let mut msg = Msg::default();
    for i in 0..v_len {
        msg.push_v((i & 0xff) as u8);
    }

    tx.send(Box::new(msg))
        .expect("crit_echo_clone: error sending msg");

    (tx, partner_tx, rx, partner_rx)
}

fn partner_box<Msg: std::marker::Send + 'static>(tx: Sender<Box<Msg>>, rx: Receiver<Box<Msg>>) {
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
                tx.send(msg).expect("echo MsgOf: error sending msg");
            });
        });
        group.bench_with_input(BenchmarkId::new("box MsgOf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner): (
                Sender<Box<MsgOf>>,
                Sender<Box<MsgOf>>,
                Receiver<Box<MsgOf>>,
                Receiver<Box<MsgOf>>,
            ) = prep_box::<MsgOf>(v_len);

            partner_box::<MsgOf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("echo box MsgOf: error receiving msg");
                //println!("echo MsgOf:    msg={}", &msg);
                tx.send(msg).expect("echo box MsgOf: error sending msg");
            });
        });
        group.bench_with_input(BenchmarkId::new("MsgMf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner) = prep::<MsgMf>(v_len);

            partner::<MsgMf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("echo MsgMf: error receiving msg");
                //println!("echo MsgMf:    msg={}", &msg);
                tx.send(msg).expect("echo MsgMf: error sending msg");
            });
        });
        group.bench_with_input(BenchmarkId::new("box MsgMf", v_len), &v_len, |b, &v_len| {
            let (tx, tx_partner, rx, rx_partner) = prep_box::<MsgMf>(v_len);

            partner_box::<MsgMf>(tx_partner, rx_partner);

            b.iter(|| {
                let msg = rx.recv().expect("echo box MsgMf: error receiving msg");
                //println!("echo MsgMf:    msg={}", &msg);
                tx.send(msg).expect("echo box MsgMf: error sending msg");
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
        group.bench_with_input(
            BenchmarkId::new("clone MsgOf", v_len),
            &v_len,
            |b, &v_len| {
                let (tx, tx_partner, rx, rx_partner) = prep::<MsgOf>(v_len);

                partner_clone::<MsgOf>(tx_partner, rx_partner);

                b.iter(|| {
                    let msg = rx.recv().expect("clone MsgOf: error receiving msg");
                    //println!("echo clone MsgOf:    msg={}", &msg);
                    tx.send(msg.clone())
                        .expect("clone MsgOf: error sending msg");
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("clone box MsgOf", v_len),
            &v_len,
            |b, &v_len| {
                let (tx, tx_partner, rx, rx_partner): (
                    Sender<Box<MsgOf>>,
                    Sender<Box<MsgOf>>,
                    Receiver<Box<MsgOf>>,
                    Receiver<Box<MsgOf>>,
                ) = prep_box::<MsgOf>(v_len);

                partner_clone_box::<MsgOf>(tx_partner, rx_partner);

                b.iter(|| {
                    let msg = rx
                        .recv()
                        .expect("echo clone box MsgOf: error receiving msg");
                    //println!("echo MsgOf:    msg={}", &msg);
                    tx.send(msg.clone())
                        .expect("clone box MsgOf: error sending msg");
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("clone MsgMf", v_len),
            &v_len,
            |b, &v_len| {
                let (tx, tx_partner, rx, rx_partner) = prep::<MsgMf>(v_len);

                partner_clone::<MsgMf>(tx_partner, rx_partner);

                b.iter(|| {
                    let msg = rx.recv().expect("echo clone MsgMf: error receiving msg");
                    //println!("echo clone MsgMf:    msg={}", &msg);
                    tx.send(msg.clone())
                        .expect("clone MsgMf: error sending msg");
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("clone box MsgMf", v_len),
            &v_len,
            |b, &v_len| {
                let (tx, tx_partner, rx, rx_partner) = prep_box::<MsgMf>(v_len);

                partner_clone_box::<MsgMf>(tx_partner, rx_partner);

                b.iter(|| {
                    let msg = rx
                        .recv()
                        .expect("echo clone box MsgMf: error receiving msg");
                    //println!("echo MsgMf:    msg={}", &msg);
                    tx.send(msg.clone())
                        .expect("clone box MsgMf: error sending msg");
                });
            },
        );
    }
}

#[allow(unused)]
fn service_manager_1000(c: &mut Criterion) {
    //println!("service_manager_1000:+");

    let plot_config = PlotConfiguration::default();

    let mut group = c.benchmark_group("one_thread");
    group.plot_config(plot_config);

    group.bench_function("1000", |b| {
        //println!("bench:+");

        let (bench_tx, bench_rx) = unbounded::<Box<MainMsgs>>();
        let main_to_bench_tx = bench_tx.clone();
        let server_to_bench_tx = bench_tx.clone();

        let (main_tx, main_rx) = unbounded::<Box<SuperProtocol>>();
        let to_main_tx = main_tx.clone();

        // "Main thread that is where the Server and Client are running."
        thread::spawn(move || {
            //println!("main:+");

            let client = Client::default();
            let server = Server::default();

            let mut service_manager = ServiceManager::default();
            service_manager.register_service(Box::new(client));
            service_manager.register_service(Box::new(server));

            //let (main_tx, main_rx) = channel::<Box<SuperProtocol>>();
            let client_tx = service_manager.get_sender(0);
            let server_tx = service_manager.get_sender(1);

            // Send client_tx and server_tx to bench
            main_to_bench_tx
                .send(Box::new(MainMsgs::ClientTx(client_tx.clone())))
                .unwrap();
            main_to_bench_tx
                .send(Box::new(MainMsgs::ServerTx(server_tx.clone())))
                .unwrap();

            // Rut server and client allowing them to process messages
            //println!("main:  Running");
            service_manager.run();

            //println!("main:-");
        });

        //println!("bench:  Get client and server tx's");
        let client_tx = bench_rx.recv().expect("Expected client_tx").client_tx();
        let server_tx = bench_rx.recv().expect("Expected server_tx").server_tx();

        b.iter(|| {
            //println!("b.iter:  Send Start");

            let msg = Box::new(SuperProtocol::P2(Pinger::Start(StartMsg {
                count: 500, // 2 messages (1 server to client, 1 client to server) so this is 1,000 messages
                main_tx: server_to_bench_tx.clone(),
                server_tx: server_tx.clone(),
                client_tx: client_tx.clone(),
            })));
            _ = server_tx.clone().send(msg);

            // Wait for server to complete
            //println!("b.iter:  wait for server to complete");
            bench_rx.recv().expect("Expected PingerDone").pinger_done();
        });

        // Stop the server and client
        //println!("bench:  Stopping server and client");
        _ = server_tx.send(Box::new(SuperProtocol::P2(Pinger::Stop)));
        _ = client_tx.send(Box::new(SuperProtocol::P1(Echo::Stop)));

        //println!("bench:-");
    });

    //println!("service_manager_1000:-");
}

criterion_group!(benches, service_manager_1000, echo, echo_clone,);
//criterion_group!(benches, service_manager_1000);
criterion_main!(benches);
