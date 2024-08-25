use hello_config::ZMQ_CONNECTION_STRING;
use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    println!("Running hello world server {}...\n", ZMQ_CONNECTION_STRING);
    assert!(responder.bind(ZMQ_CONNECTION_STRING).is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}
