#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

type Message = String;

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)] 
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?} status message is {:?} ", sat_id, StatusMessage::Ok);
    sat_id
}

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message ) {
        to.mailbox.messages.push(msg);
    }
}

impl CubeSat {
    fn rcv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

fn main() {

    let base = GroundStation {};
    let mut sat_a = CubeSat { id: 0, mailbox: Mailbox { messages: vec![ ] } };
    println!("to: {:?}", sat_a);
    base.send(&mut sat_a, Message::from("Hello there!"));
    if let Some(message) = sat_a.rcv() {
        println!("{:?}", message);
    }

}
