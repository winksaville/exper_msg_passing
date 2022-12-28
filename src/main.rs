use std::{thread, sync::{mpsc::{Sender, channel, Receiver}, RwLock, Arc}};

use exper_msg_passing::{Client, Pinger, Server, ServiceManager, StartMsg, SuperProtocol};

pub struct SenderWrapper<T>(Sender<T>);

//impl<T> SenderWrapper<T> {
//    fn new(sender: Sender<T>) -> Self {
//        Self(sender)
//    }
//}
unsafe impl<T> Send for SenderWrapper<T> {}
unsafe impl<T> Sync for SenderWrapper<T> {}

fn main() {
    //let client = Client::default();
    //let mut client_service_manager = ServiceManager::default();
    //client_service_manager.register_service(Box::new(client));

    //let server = Server::default();
    //let mut server_service_manager = ServiceManager::default();
    //server_service_manager.register_service(Box::new(client));

    //let client_tx = client_service_manager.get_sender(0);
    //let server_tx = server_service_manager.get_sender(0);

    //let mut client_tx: Arc<RwLock<Option<Sender<Box<SuperProtocol>>>>> =
    //    Arc::new(RwLock::new(None));

    // This would be better shared via a message but this is "quick/dirty" for now.
    let client_tx: Arc<RwLock<Option<SenderWrapper<Box<SuperProtocol>>>>> =
        Arc::new(RwLock::new(None));

    //for _ in 0..2 {

        let client_thread_handle = thread::spawn(move || {
            let client = Client::default();
            let mut client_service_manager = ServiceManager::default();
            client_service_manager.register_service(Box::new(client));

            let c_tx = client_service_manager.get_sender(0);

            let x = client_tx.clone();
            let mut writer_lock_client_tx = x.write().unwrap();
            let sw = SenderWrapper(c_tx);
            *writer_lock_client_tx = Some(sw);

            // Invoke run so client can process messages
            client_service_manager.run();

            client_service_manager.enable_running();
        });

        let server_thread_handle = thread::spawn(move || {
            // Wait for client_tx to be Some
            let c_tx_sw = loop {
                let x = client_tx.clone();
                let reader_lock_client_tx = x.read().unwrap();
                if let Some(sw) = *reader_lock_client_tx {
                    break sw
                }
            };

            let server = Server::default();
            let mut server_service_manager = ServiceManager::default();
            server_service_manager.register_service(Box::new(server));

            let s_tx = server_service_manager.get_sender(0);

            let msg = Box::new(SuperProtocol::P2(Pinger::Start(StartMsg {
                count: 1,
                client_tx: Some((c_tx_sw.0).clone()),
                server_tx: Some(s_tx.clone()),
            })));
            _ = server_service_manager.get_sender(0).send(msg);

            // Invoke run so server can process messages
            server_service_manager.run();

            server_service_manager.enable_running();
        });

        client_thread_handle.join();
        //server_thread_handle.join();
    //}
}
