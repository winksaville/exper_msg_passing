use crossbeam_channel::unbounded;
use std::thread;

use exper_msg_passing::{
    Client, Echo, MainMsgs, Pinger, Server, ServiceManager, StartMsg, SuperProtocol,
};

fn main() {
    println!("main:+");
    // Create a channel so client can send its a tx channel to main

    let (main_tx, main_rx) = unbounded::<Box<SuperProtocol>>();

    let client_to_main_tx = main_tx.clone();
    let server_to_main_tx = main_tx.clone();

    let client_thread_handle = thread::spawn(move || {
        println!("client_thread:+");
        let client = Client::default();
        let mut client_service_manager = ServiceManager::default();
        client_service_manager.register_service(Box::new(client));
        println!("client_thread:  registered with client_service_manager");

        // Send back our client_tx so it will be passed to server
        let client_tx = client_service_manager.get_sender(0);
        client_to_main_tx
            //.clone()
            .send(Box::new(SuperProtocol::P3(MainMsgs::ClientTx(client_tx))))
            .unwrap();
        //main_tx.clone().send(Box::new(SuperProtocol::P3(MainMsgs::ClientTx(client_tx)))).unwrap();
        println!("client_thread:  Sent client_tx to main");

        // Invoke run so client can process messages
        println!("client_thread:  Invoke run");
        client_service_manager.run();

        println!("client_thread:-");
    });

    // Receive MainMsgs::ClientTx msg
    let client_tx = match main_rx.recv() {
        Ok(msg) => match *msg {
            SuperProtocol::P3(MainMsgs::ClientTx(c_tx)) => c_tx,
            _ => panic!("main: Unexpected msg={msg:?}"),
        },
        Err(why) => panic!("main: Error receiving client_tx, why={why}"),
    };
    println!("main:  client_tx={client_tx:?}");

    let server_thread_handle = thread::spawn(move || {
        println!("server_thread:+");

        let server = Server::default();
        let mut server_service_manager = ServiceManager::default();
        server_service_manager.register_service(Box::new(server));
        println!("server_thread:  registered with server_service_manager");

        // Send back our server_tx so we can be be started and stopped
        let server_tx = server_service_manager.get_sender(0);
        server_to_main_tx
            //.clone()
            .send(Box::new(SuperProtocol::P3(MainMsgs::ServerTx(server_tx))))
            .unwrap();
        println!("server_thread:  Sent server_tx to main");

        println!("server_thread:  Invoke run");
        server_service_manager.run();

        println!("server_thread:-");
    });

    // Receive MainMsgs::ServerTx msg
    let server_tx = match main_rx.recv() {
        Ok(msg) => match *msg {
            SuperProtocol::P3(MainMsgs::ServerTx(c_tx)) => c_tx,
            _ => panic!("main: Unexpected msg={msg:?}"),
        },
        Err(why) => panic!("main: Error receiving server_tx, why={why}"),
    };
    println!("main:  server_tx={server_tx:?}");

    for _ in 0..2 {
        let msg = Box::new(SuperProtocol::P2(Pinger::Start(StartMsg {
            count: 3,
            main_tx: main_tx.clone(),
            server_tx: server_tx.clone(),
            client_tx: client_tx.clone(),
        })));
        _ = server_tx.send(msg);

        // Wait for server to complete
        match main_rx.recv() {
            Ok(msg) => match &*msg {
                SuperProtocol::P2(Pinger::Done) => println!("main: server is Done"),
                _ => panic!("main: Unexpected msg={msg:?}"),
            },
            Err(why) => panic!("main: Error receiving server_tx, why={why}"),
        }
    }

    // Stop the server and clients
    println!("main: Stopping server and client");
    _ = server_tx.send(Box::new(SuperProtocol::P2(Pinger::Stop)));
    _ = client_tx.send(Box::new(SuperProtocol::P1(Echo::Stop)));

    println!("main: Waiting for server and client to stop");
    _ = server_thread_handle.join();
    _ = client_thread_handle.join();

    println!("main:-");
}
