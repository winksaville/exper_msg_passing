use std::sync::mpsc::{channel, Receiver, Sender};

// -------------------------

#[derive(Debug)]
#[allow(unused)]
pub enum Echo {
    Echo(Sender<Box<SuperProtocol>>),
    Stop,
}

#[derive(Default, Clone)]
pub struct Client;

impl HandleMessage<SuperProtocol> for Client {
    #[allow(unused)]
    fn handle_message(&mut self, msg: Box<SuperProtocol>) -> ServiceState {
        match *msg {
            SuperProtocol::P1(Echo::Echo(ref sender)) => {
                //println!("Client::handle_message: Echoing");
                // How can I remove the need for the clone() here,
                // I'd like to just do `sender.send(msg);`?
                if sender.clone().send(msg).is_ok() {
                    ServiceState::Running
                } else {
                    ServiceState::Stopped
                }
            }
            SuperProtocol::P1(Echo::Stop) => {
                //println!("Client::handle_message: STOP msg={msg:?}");
                ServiceState::Stopped
            }
            _ => {
                //println!("Client::handle_message: Unknown msg={msg:?}");
                ServiceState::Running
            }
        }
    }
}

// -------------------------
#[derive(Debug)]
pub struct Work2Msg {
    pub f1: u32,
}

#[derive(Debug)]
pub struct StartMsg {
    pub count: usize,
    pub main_tx: Sender<Box<SuperProtocol>>,
    pub server_tx: Sender<Box<SuperProtocol>>,
    pub client_tx: Sender<Box<SuperProtocol>>,
}

#[derive(Debug)]
pub enum Pinger {
    Start(StartMsg),
    Done,
    Stop,
}

#[derive(Default, Clone, Debug)]
#[allow(unused)]
pub struct Server {
    pub count: usize,
    // Having this as Option is ugly
    main_tx: Option<Sender<Box<SuperProtocol>>>,
    client_tx: Option<Sender<Box<SuperProtocol>>>,
    my_tx: Option<Sender<Box<SuperProtocol>>>,
    running: ServiceState,
}

impl Server {
    fn send_to_client(&self, msg: Box<SuperProtocol>) -> ServiceState {
        //println!("Server::send_to_client:+");
        #[allow(clippy::let_and_return)]
        let r = if let Some(c_tx) = self.client_tx.clone() {
            if c_tx.send(msg).is_ok() {
                //println!("Server::send_to_client");
                ServiceState::Running
            } else {
                ServiceState::Stopped
            }
        } else {
            ServiceState::Stopped
        };
        //println!("Server::send_to_client:-r={r:?}");
        r
    }

    fn send_to_main(&self, msg: Box<SuperProtocol>) -> ServiceState {
        //println!("Server::send_to_main:+");
        #[allow(clippy::let_and_return)]
        let r = if let Some(c_tx) = self.main_tx.clone() {
            if c_tx.send(msg).is_ok() {
                //println!("Server::send_to_main");
                ServiceState::Running
            } else {
                ServiceState::Stopped
            }
        } else {
            ServiceState::Stopped
        };
        //println!("Server::send_to_main:- r={r:?}");
        r
    }
}

impl HandleMessage<SuperProtocol> for Server {
    fn handle_message(&mut self, msg: Box<SuperProtocol>) -> ServiceState {
        match self.running {
            ServiceState::Running => {
                match &*msg {
                    SuperProtocol::P2(Pinger::Start(StartMsg {
                        count,
                        main_tx,
                        server_tx,
                        client_tx,
                    })) => {
                        //println!("Server::handle_message: Start msg={msg:?}");
                        self.count = *count;
                        self.main_tx = Some(main_tx.clone());
                        self.my_tx = Some(server_tx.clone());
                        self.client_tx = Some(client_tx.clone());

                        if self.count > 0 {
                            self.send_to_client(Box::new(SuperProtocol::P1(Echo::Echo(
                                self.my_tx.clone().unwrap(),
                            ))))
                        } else {
                            // We're not pinging because count is zero initially, so no pinging
                            //println!("Server::handle_message: StartMsg::count == {} so no pinging, sending DONE msg={msg:?}", self.count);
                            self.send_to_main(Box::new(SuperProtocol::P2(Pinger::Done)))
                        }
                    }
                    SuperProtocol::P2(Pinger::Stop) => {
                        //println!("Server::handle_message: STOP msg={msg:?}");
                        ServiceState::Stopped
                    }
                    SuperProtocol::P1(Echo::Echo(_)) => {
                        if self.count > 0 {
                            self.count -= 1;
                            //println!("Server::handle_message: Echo count={} received msg={msg:?}", self.count);
                            self.send_to_client(msg)
                        } else {
                            //println!("Server::handle_message: Echo count={} DONE msg={msg:?}", self.count);
                            self.send_to_main(Box::new(SuperProtocol::P2(Pinger::Done)))
                        }
                    }
                    _ => {
                        //println!("Server::handle_message: ignore msg={msg:?}");
                        ServiceState::Running
                    }
                }
            }
            ServiceState::Stopped => {
                println!("Server::handle_message: Stopped");
                self.send_to_client(Box::new(SuperProtocol::P1(Echo::Stop)));
                ServiceState::Stopped
            }
        }
    }
}
// -------------------------

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ServiceState {
    Running,
    Stopped,
}

impl Default for ServiceState {
    fn default() -> Self {
        ServiceState::Running
    }
}

#[derive(Debug)]
#[allow(unused)]
pub enum MainMsgs {
    ClientTx(Sender<Box<SuperProtocol>>),
    ServerTx(Sender<Box<SuperProtocol>>),
    //StartAndRun,
}

// This isn't what I want, but this compiles.
//
// The closest safe rust looks to be `std::any::Any`,
// I'll try that next
#[derive(Debug)]
pub enum SuperProtocol {
    P1(Echo),
    P2(Pinger),
    P3(MainMsgs),
}

pub trait HandleMessage<SuperProtocol> {
    fn handle_message(&mut self, msg: Box<SuperProtocol>) -> ServiceState;
}

#[allow(unused)]
pub struct ServiceRec {
    running: ServiceState,
    tx: Sender<Box<SuperProtocol>>,
    rx: Receiver<Box<SuperProtocol>>,

    message_handler: Box<dyn HandleMessage<SuperProtocol>>,
}

#[derive(Default)]
pub struct ServiceManager {
    pub services: Vec<ServiceRec>,
}

#[allow(unused)]
impl ServiceManager {
    pub fn register_service(&mut self, message_handler: Box<dyn HandleMessage<SuperProtocol>>) {
        let (tx, rx) = channel();
        let sr = ServiceRec {
            running: ServiceState::Running,
            tx,
            rx,
            message_handler,
        };
        self.services.push(sr);
    }

    pub fn get_sender(&self, idx: usize) -> Sender<Box<SuperProtocol>> {
        self.services[idx].tx.clone()
    }

    pub fn dispatch_message(&mut self, idx: usize, msg: Box<SuperProtocol>) -> ServiceState {
        let ss = self.services[idx].message_handler.handle_message(msg);
        self.services[idx].running = ss.clone();

        ss
    }

    pub fn run(&mut self) {
        //println!("ServiceManager::run:+");
        loop {
            let mut running_count = self.services.len();
            //println!("ServiceManager::run: TOW running_count={running_count}, services.len={}", self.services.len());
            for idx in 0..self.services.len() {
                //println!("ServiceManager::run: TOF idx={idx}");
                let service = &mut self.services[idx];
                if service.running == ServiceState::Running {
                    if let Ok(msg) = service.rx.try_recv() {
                        //println!("ServiceManager::run: services[{idx}] dispatch msg={msg:?}");
                        // Can't call self.dispatch_message because borrow checker says:
                        //   "cannot borrow `*self` as mutable more than once at a time"
                        //service.running = self.dispatch_message(idx, msg);
                        service.running = service.message_handler.handle_message(msg);
                        if service.running != ServiceState::Running {
                            running_count -= 1;
                            //println!("ServiceManager::run: services[{idx}] NOT running");
                        }
                        //} else {
                        //    //println!("ServiceManager::run: services[{idx}] no messages");
                    }
                } else {
                    running_count -= 1;
                    //println!("ServiceManager::run: services[{idx}] is STOPPED");
                }
            }
            if running_count == 0 {
                break;
            }
            //if running_count > 0 {
            //    std::thread::sleep(std::time::Duration::from_secs(1));
            //}
        }
        //println!("ServiceManager::run:-");
    }
}

#[allow(unused)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_handle_message() {
        let mut client = Client::default();
        let msg = Box::new(SuperProtocol::P1(Echo::Stop));
        let ss = client.handle_message(msg);
        assert_eq!(ss, ServiceState::Stopped);

        let mut server = Server::default();
        let msg = Box::new(SuperProtocol::P2(Pinger::Stop));
        let ss = server.handle_message(msg);
        assert_eq!(ss, ServiceState::Stopped);
    }

    #[test]
    fn test_service_manager_initialization() {
        let mut service_manager = ServiceManager::default();

        let mut client = Client::default();
        let mut server = Server::default();

        service_manager.register_service(Box::new(client));
        service_manager.register_service(Box::new(server));

        for idx in 0..service_manager.services.len() {
            assert_eq!(service_manager.services[idx].running, ServiceState::Running);
        }
    }

    #[test]
    fn test_dispatch_message() {
        let mut service_manager = ServiceManager::default();

        let mut client = Client::default();
        let mut server = Server::default();

        service_manager.register_service(Box::new(client));
        service_manager.register_service(Box::new(server));

        // Verify client is running and then Stops
        assert_eq!(service_manager.services[0].running, ServiceState::Running);
        let msg = Box::new(SuperProtocol::P1(Echo::Stop));
        service_manager.dispatch_message(0, msg);
        assert_eq!(service_manager.services[0].running, ServiceState::Stopped);

        // Verify we can send Start and its Running
        let msg = Box::new(SuperProtocol::P2(Pinger::Stop));
        service_manager.dispatch_message(1, msg);
        assert_eq!(service_manager.services[1].running, ServiceState::Stopped);
    }

    #[test]
    fn test_run_1() {
        let mut client = Client::default();
        let mut server = Server::default();

        let mut service_manager = ServiceManager::default();
        service_manager.register_service(Box::new(client));
        service_manager.register_service(Box::new(server));

        let (main_tx, main_rx) = channel::<Sender<Box<SuperProtocol>>>();
        let client_tx = service_manager.get_sender(0);
        let server_tx = service_manager.get_sender(1);

        // All services are running
        for idx in 0..service_manager.services.len() {
            assert_eq!(service_manager.services[idx].running, ServiceState::Running);
        }

        let msg = Box::new(SuperProtocol::P2(Pinger::Start(StartMsg {
            count: 1,
            main_tx: server_tx.clone(),
            server_tx: server_tx.clone(),
            client_tx: client_tx.clone(),
        })));
        service_manager.get_sender(1).send(msg);

        // Invoke run so server and client can process messages
        service_manager.run();

        // All services are stopped
        for idx in 0..service_manager.services.len() {
            assert_eq!(service_manager.services[idx].running, ServiceState::Stopped);
        }
    }

    #[test]
    fn test_rerun_twice() {
        let mut client = Client::default();
        let mut server = Server::default();

        let mut service_manager = ServiceManager::default();
        service_manager.register_service(Box::new(client));
        service_manager.register_service(Box::new(server));

        let (main_tx, main_rx) = channel::<Sender<Box<SuperProtocol>>>();
        let client_tx = service_manager.get_sender(0);
        let server_tx = service_manager.get_sender(1);

        for _ in 0..2 {
            let msg = Box::new(SuperProtocol::P2(Pinger::Start(StartMsg {
                count: 1,
                main_tx: server_tx.clone(),
                server_tx: server_tx.clone(),
                client_tx: client_tx.clone(),
            })));
            _ = service_manager.get_sender(1).send(msg);

            // Invoke run so server and client can process messages
            service_manager.run();
        }

        // Stop the server and clients
        println!("main: Stop server and client");
        _ = server_tx.send(Box::new(SuperProtocol::P2(Pinger::Stop)));
        _ = client_tx.send(Box::new(SuperProtocol::P1(Echo::Stop)));

        for idx in 0..service_manager.services.len() {
            assert_eq!(service_manager.services[idx].running, ServiceState::Stopped);
        }
    }
}
