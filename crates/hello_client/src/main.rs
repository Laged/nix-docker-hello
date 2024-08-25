use hello_config::ZMQ_CONNECTION_STRING;

fn main() {
    println!(
        "Connecting to hello world server {}...\n",
        ZMQ_CONNECTION_STRING
    );

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect(ZMQ_CONNECTION_STRING).is_ok());

    let mut msg = zmq::Message::new();

    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        requester.send("Hello", 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}
