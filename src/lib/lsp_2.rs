trait Bird {
    fn fly(&self);
}

struct Sparrow;

impl Bird for Sparrow {
    fn fly(&self) {
        println!("Sparrow flying...");
    }
}

struct Ostrich;

impl Bird for Ostrich {
    fn fly(&self) {
        println!("Ostrich can't fly.");
    }
}
