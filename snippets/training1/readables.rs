pub struct Magazine {
    name : String,
    headline: String
}

pub struct Book {
    name: String,
    author: String,
    pages: u32
}

pub struct Message {
    title: String,
    content: String
}

pub trait Readable {
    fn print(&self) {
        println!("{}", self.to_string());
    }

    fn to_string(&self) -> String;
}

impl Magazine {
    pub fn new(name : String, headline: String) -> Magazine {
        Magazine {
            name,
            headline
        }
    }
}

impl Readable for Magazine {
    fn to_string(&self) -> String {
        format!("Magazine {} with a headline of {}",
            self.name, self.headline)
    }
}

impl Book {
    pub fn new(name: String, author: String, pages: u32) -> Book {
        Book {
            name,
            author,
            pages
        }
    }
}

impl Readable for Book {
    fn to_string(&self) -> String {
        format!("{} by {} with {} pages",
            self.name, self.author, self.pages)
    }
}

impl Message {
    pub fn new(title : String, content: String) -> Message {
        Message {
            title,
            content
        }
    }
}

impl Readable for Message {
    fn to_string(&self) -> String {
        format!("{} :: {}", self.title, self.content)
    }
}