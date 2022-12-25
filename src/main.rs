use exper_msg_passing::{Client, Pinger, Server, ServiceManager, StartMsg, SuperProtocol};

fn main() {
    let client = Client::default();
    let server = Server::default();

    let mut service_manager = ServiceManager::default();
    service_manager.register_service(Box::new(client));
    service_manager.register_service(Box::new(server));

    let client_tx = service_manager.get_sender(0);
    let server_tx = service_manager.get_sender(1);

    for _ in 0..2 {
        let msg = Box::new(SuperProtocol::P2(Pinger::Start(StartMsg {
            count: 1,
            client_tx: Some(client_tx.clone()),
            server_tx: Some(server_tx.clone()),
        })));
        _ = service_manager.get_sender(1).send(msg);

        // Invoke run so server and client can process messages
        service_manager.run();

        // Re-enable running, works for this simple test
        service_manager.enable_running();
    }
}
