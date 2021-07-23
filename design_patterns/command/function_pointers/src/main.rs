//Approach: using function pointers
// Creating each individual comman as a different function and store fiunction pointers
// to invoke functions at a different time

type FnPtr = fn() -> String;
struct Command{
    execute: FnPtr,
    rollback: FnPtr,
}

struct Schema {
    commands: Vec<Command>
}

impl Schema {
    fn new() -> Self {
        Self {commands: vec![]}
    }
    fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
        self.commands.push(Command{execute, rollback});
    }
    fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }
    fn rollback(&self) -> Vec<String> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| (cmd.rollback)())
            .collect()
    }
}

fn add_field() -> String {
    "add field".to_string()
}

fn remove_field() -> String {
    "remove field".to_string()
}


fn main() {
    println!("Hello, world!");
}
