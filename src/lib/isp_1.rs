trait Worker {
    fn work(&self);
    fn eat(&self);
}

struct Engineer;

impl Worker for Engineer {
    fn work(&self) {
        // logic for engineer's work
    }

    fn eat(&self) {
        // logic for engineer's eating habits
    }
}
