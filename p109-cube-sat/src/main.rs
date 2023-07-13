#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

struct GroundStation;

#[derive(Debug, PartialEq)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for (idx, message) in self.messages.iter().enumerate() {
            if message.to == recipient.id {
                return Some(self.messages.remove(idx));
            }
        }
        None
    }
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {

    let mut mailbox = Mailbox { messages: vec![]};
    let groundStation = GroundStation {};
    for id in fetch_sat_ids() {
        let current_cube = groundStation.connect(id );
        let message  = Message { to: id, content: String::from("Hello there!") };
        groundStation.send(&mut mailbox, message);
        let message = current_cube.recv(&mut mailbox);
        println!("Message to CubeSat {:?}: {:?} ", current_cube, message.unwrap());
    }


}
