trait Worker {
    fn work(&self);
}

trait Eater {
    fn eat(&self);
}

struct Engineer;

impl Worker for Engineer {
    fn work(&self) {
        // logic for engineer's work
    }
}

struct Manager;

impl Worker for Manager {
    fn work(&self) {
        // logic for manager's work
    }
}

impl Eater for Manager {
    fn eat(&self) {
        // logic for manager's eating habits
    }
}
