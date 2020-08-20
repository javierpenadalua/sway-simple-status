use std::process::Command;

pub struct Volume {
    pub value: String,
    dt: u32
}

impl Volume {

    pub fn new() -> Volume {
        let mut v = Volume {
            value: String::new(),
            dt: 0
        };

        v.update0();

        return v;
    }

    pub fn update(&mut self) {
        
        self.dt = (self.dt + 1) % 2;

        if self.dt == 0 {
            self.update0();
        }
    }
    
    fn update0(&mut self) {

        let mut cmd = Command::new("pamixer");

        cmd.arg("--get-volume-human");

        let output = cmd.output().unwrap().stdout;
        
        let svolume = String::from_utf8(output).unwrap();
        let svolume = svolume.trim();

        self.value = if svolume == "muted" {
            format!("<span foreground=\"red\"> </span>")
        } else {
            format!("<span foreground=\"yellow\"> {}</span>", svolume)
        };
    }
}
