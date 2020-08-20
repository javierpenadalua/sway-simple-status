mod date;
mod battery;
mod volume;

use std::{thread, time};

fn main() {
    
    let mut date_obj = date::Date::new();
    let mut battery_obj = battery::Battery::new();
    let mut volume_obj = volume::Volume::new();

    loop {
        
        date_obj.update();
        battery_obj.update();
        volume_obj.update();

        println!("{}   {}   {}", battery_obj.value, volume_obj.value, date_obj.value);

        thread::sleep(time::Duration::from_millis(100));
    }
}
