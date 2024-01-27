

struct  Person {
    name : &str,
    genger: &str,
}


impl Person {
    fn new(name:&str, genger:&str) -> Person {
        Person {
            name,
            genger,
        }
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn get_genger(&self) -> &str {
        self.genger
    }
}