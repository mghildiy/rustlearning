use std::fmt::{Display, Formatter};
use rand::{thread_rng, Rng};

const quote: &str = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";
pub fn search_text(to_search: &str) {
    for (line_no, line) in quote.lines().enumerate() {
        if line.contains(to_search) {
            println!("line: \"{}\" , line number: {}", line, line_no+1);
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "File name: {}, state: {:?}", self.name, self.state)
    }
}

impl File {
    pub fn from(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed
        }
    }
    pub fn from_data(name: &str, data: &Vec<u8>) -> File {
        let mut file = File::from(name);
        file.set_data(data.clone());
        file
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut temp = self.data.clone();
        save_to.reserve(temp.len());
        save_to.append(&mut temp);
        Ok(save_to.len())
    }
}

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}
pub fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

pub fn close(mut f: File) -> Result<File, String> {
   f.state = FileState::Closed;
   Ok(f)
}

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED")
        }
    }
}

// pub type Message = String;

#[derive(Debug)]
pub struct Message {
    to: u64,
    message: String
}

impl Message {
    pub fn from(to: u64, m: &str) -> Message {
        Message {
            to: to,
            message: String::from(m)
        }
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Message {
            to: self.to,
            message: self.message.clone()
        }
    }
}

#[derive(Debug)]
pub struct Mailbox {
    pub messages: Vec<Message>
}

impl Clone for Mailbox {
    fn clone(&self) -> Self {
        Mailbox {
            messages: self.messages.clone()
        }
    }
}

impl Mailbox {
    pub fn post(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn deliver(&mut self, sat: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == sat.id {
                let message = self.messages.remove(i);
                return Some(message);
            }
        }
        None
    }

}

pub struct CubeSat {
    pub id: u64,
    mailbox: Mailbox
}

impl CubeSat {
    pub fn recv(&mut self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(self)
    }
}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        CubeSat {
            id: self.id,
            mailbox: self.mailbox.clone()
        }
    }
}

#[derive(Debug)]
pub enum StatusMessage {
    Ok,
    NotOk
}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        self.clone()
    }
}

pub struct GroundStation;

impl GroundStation {
    pub fn send(&self, mailbox: &mut Mailbox, m: Message) {
        mailbox.post(m)
    }

    pub fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id, mailbox: Mailbox { messages: vec![] } }
    }
}

pub fn check_status(sat: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

pub fn fetch_sat_ids() -> Vec<u64> {
    vec![1,2,3]
}

#[derive(Copy, Clone)]
pub struct PointCopy {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct PointClone {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Person {
    pub name: String,
    pub age: u16
}

