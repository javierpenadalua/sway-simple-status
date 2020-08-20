use std::fs;

pub struct Battery {
    pub value: String
}

impl Battery {
    pub fn new() -> Battery {
        let mut b = Battery {value: String::new()};

        b.update();

        return b;
    }

    pub fn update(&mut self) {
        let content = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap();
        let percentage : u32 = content.trim().parse().unwrap();
        let mut color = "green";

        if percentage < 15 {
            color = "red";
        }

        self.value = format!("<span foreground=\"{}\"> {}%</span>", color, percentage);

    }
}
