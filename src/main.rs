mod date;
mod battery;

use std::{thread, time};

fn main() {
    
    let mut date_obj = date::Date::new();
    let mut battery_obj = battery::Battery::new();

    loop {
        
        date_obj.update();
        battery_obj.update();

        println!("{} {}", battery_obj.value, date_obj.value);

        thread::sleep(time::Duration::from_millis(1000));
    }
}
