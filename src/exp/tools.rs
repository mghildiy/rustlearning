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
    pub data: Vec<u8>
}

impl File {
    pub fn from(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new()
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn read(&self, save_to: &mut Vec<u8>) -> usize {
        let mut temp = self.data.clone();
        save_to.reserve(temp.len());
        save_to.append(&mut temp);
        save_to.len()
    }
}