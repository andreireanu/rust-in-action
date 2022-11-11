#![allow(unused_variables)]

#[derive(Debug)] 
struct CubeSat {
    id: u64,
}

#[derive(Debug)] 
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

struct GroundStation {}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if recipient.id == self.messages[i].to  {
                let message = self.messages.remove(i);
                return Some(message);
            }
        };
        return None
    }
}

fn main() {

    let mut cubeVec: Vec<&CubeSat> = Vec::new();
    let c1 = CubeSat { id: 1 };
    let mut c2 = CubeSat { id: 2 };
    let mut c3 = CubeSat { id: 3 };
    cubeVec.push(&c1);
    cubeVec.push(&c2);
    cubeVec.push(&c3);
    let mut mailbox = Mailbox { messages: Vec::new()};
    mailbox.post( Message { to: 1, content: String::from("Message for 1") }) ;  
    let msg1 = mailbox.deliver(c1).unwrap();
    println!("{:?}", msg1);
    mailbox.post( Message { to: 2, content: String::from("Message for 2") });
    mailbox.post( Message { to: 3, content: String::from("Message for 3") });


}
