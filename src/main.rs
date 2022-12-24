use std::num::Wrapping;

use exper_msg_passing::{Protocol1, Service1, ServiceManager, SuperProtocol};

fn main() {
    let mut service_manager = ServiceManager { services: vec![] };

    let mut service1 = Service1::default();
    service1.info.push(Wrapping(127));

    service_manager.register_service(Box::new(service1));

    let msg = Box::new(SuperProtocol::P1(Protocol1::Init));
    service_manager.dispatch_message(0, msg);
}
