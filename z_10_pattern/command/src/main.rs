use std::collections::HashMap;

trait Command {
    fn execute(&self);
}

struct NullCommand;
impl Command for NullCommand {
    fn execute(&self) {
        println!("Do nothing");
    }
}

#[derive(Copy, Clone)]
struct TV;
impl TV {
    fn new() -> TV {
        TV
    }
    fn on(&self) {
        println!("TV is on, watch movies.");
    }
    fn off(&self) {
        println!("TV is off");
    }
}

struct TVOnCommand {
    tv: TV,
}
impl TVOnCommand {
    fn new(tv: TV) -> TVOnCommand {
        TVOnCommand { tv }
    }
}
impl Command for TVOnCommand {
    fn execute(&self) {
        self.tv.on();
    }
}

struct TVOffCommand {
    tv: TV,
}
impl TVOffCommand {
    fn new(tv: TV) -> TVOffCommand {
        TVOffCommand { tv }
    }
}
impl Command for TVOffCommand {
    fn execute(&self) {
        self.tv.off();
    }
}

struct TVRemoteControl {
    command: HashMap<i32, Box<dyn Command>>,
    null_command: NullCommand,
}
impl TVRemoteControl {
    fn new() -> TVRemoteControl {
        TVRemoteControl {
            command: HashMap::new(),
            null_command: NullCommand {},
        }
    }
    fn set_command(&mut self, idx: i32, cmd: Box<dyn Command>) {
        self.command.insert(idx, cmd);
    }
    fn press_button(&self, idx: i32) {
        if let Some(cmd) = self.command.get(&idx) {
            cmd.execute();
        } else {
            self.null_command.execute();
        }
    }
}

fn main() {
    let tv = TV::new();
    let mut remote = TVRemoteControl::new();
    remote.press_button(0);

    remote.set_command(1, Box::new(TVOnCommand::new(tv)));
    remote.set_command(2, Box::new(TVOffCommand::new(tv)));
    remote.press_button(1);
    remote.press_button(2);
}
