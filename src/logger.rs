use std::fs;

pub struct CSVLogger {
    text: String
}

impl CSVLogger {
    pub fn new() -> CSVLogger {
        CSVLogger { text: String::from("") }
    }

    pub fn write_line(&mut self, line: String) {
        self.text += &line;
        self.text += "\n";
    }

    pub fn write(&mut self, text: String) {
        self.text += &text;
    }

    pub fn flush(&mut self) {
        self.text.clear();
    }

    pub fn to_file(&self, filename: &str) {
        fs::write(String::from("out/") + &filename, self.text.clone()).expect("Could not write file");
    }

    pub fn print(&self) {
        print!("{}", self.text);
    }
}

#[macro_export]
macro_rules! csv_line {
    ($logger:ident, $y:expr, $($x:expr),*) => {
        {
            $logger.write_line(format!("{}", $y) $( + "," + format!("{}", $x).as_str())*)
        }
    };
}

#[macro_export]
macro_rules! csv_logger {
    ($y:expr, $($x:expr),*) => {
        {
            let mut logger = CSVLogger::new();
            logger.write_line(format!("{}", $y) $( + "," + format!("{}", $x).as_str())*);
            logger
        }
    };
}