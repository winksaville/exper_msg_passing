use std::num::Wrapping;

// -------------------------
#[allow(unused)]
#[derive(Debug)]
pub struct Work1Msg {
    f1: u32,
}

#[derive(Debug)]
pub enum Protocol1 {
    Init,
    Work1(Work1Msg),
}

#[derive(Default, Clone)]
pub struct Service1 {
    pub info: Vec<Wrapping<i8>>,
}

impl HandleMessage<SuperProtocol> for Service1 {
    fn handle_message(&mut self, msg: Box<SuperProtocol>) {
        self.info[0] += 1;
        println!(
            "MyService::handle_message: self.info={:?} msg={msg:?}",
            self.info
        );
    }
}

// -------------------------
#[derive(Debug)]
pub struct Work2Msg {
    pub f1: u32,
}

#[derive(Debug)]
pub enum Protocol2 {
    Init,
    Work2(Work2Msg),
}

#[derive(Default, Clone)]
pub struct Service2 {
    pub info: Vec<Wrapping<u8>>,
}

impl HandleMessage<SuperProtocol> for Service2 {
    fn handle_message(&mut self, msg: Box<SuperProtocol>) {
        self.info[0] -= 1;
        println!(
            "MyService::handle_message: self.info={:?} msg={msg:?}",
            self.info
        );
    }
}
// -------------------------

#[derive(Debug)]
pub enum SuperProtocol {
    P1(Protocol1),
    P2(Protocol2),
}

pub trait HandleMessage<SuperProtocol> {
    fn handle_message(&mut self, msg: Box<SuperProtocol>);
}

#[allow(unused)]
pub struct ServiceRec {
    // This isn't what I want, but this compiles. What I'd like
    // to have is that this be an array over services that can
    // handle any type of protocol. What this requires is that
    // all services need to handle every type of message. Definitely
    // not what I want.
    message_handler: Box<dyn HandleMessage<SuperProtocol>>,
}

pub struct ServiceManager {
    pub services: Vec<ServiceRec>,
}

#[allow(unused)]
impl ServiceManager {
    pub fn register_service(&mut self, message_handler: Box<dyn HandleMessage<SuperProtocol>>) {
    //fn register_service(&mut self, message_handler: Box<dyn HandleMessage>) {
        let sr = ServiceRec { message_handler };
        self.services.push(sr);
    }

    pub fn dispatch_message(&mut self, idx: usize, msg: Box<SuperProtocol>) {
        let h = &mut self.services[idx].message_handler;
        h.handle_message(msg);
    }
}

#[allow(unused)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_handle_message() {
        let mut service1 = Service1::default();
        service1.info.push(Wrapping(127));
        let msg = Box::new(SuperProtocol::P1(Protocol1::Init));
        service1.handle_message(msg);

        let mut service2 = Service2::default();
        service2.info.push(Wrapping(0));
        let msg = Box::new(SuperProtocol::P2(Protocol2::Init));
        service2.handle_message(msg);
    }

    #[test]
    fn test_dispatch_message() {
        let mut service1 = Service1::default();
        service1.info.push(Wrapping(127));

        let mut service2 = Service1::default();
        service2.info.push(Wrapping(0));

        let mut service_manager = ServiceManager { services: vec![] };
        service_manager.register_service(Box::new(service1));
        service_manager.register_service(Box::new(service2));

        let msg = Box::new(SuperProtocol::P1(Protocol1::Init));
        service_manager.dispatch_message(0, msg);

        let msg = Box::new(SuperProtocol::P1(Protocol1::Init));
        service_manager.dispatch_message(1, msg);
    }
}
