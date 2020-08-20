extern crate chrono;

use chrono::Local;

pub struct Date {
    pub value: String
}

impl Date {
    pub fn new() -> Date {
        let mut d = Date {value: String::new()};
        
        d.update();

        return d;
    }

    pub fn update(&mut self) {
        let date = Local::now();
        let sdate = date.format("%Y-%m-%d %H:%M:%S");

        self.value = format!(" {}", sdate);
    }
}
