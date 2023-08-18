struct Bird;

impl Bird {
    fn fly(&self) {
        println!("Flying...");
    }
}

struct Ostrich;

impl Bird {
    fn fly(&self) {
        // Incorrect implementation for an ostrich
        println!("Ostrich running...");
    }
}
