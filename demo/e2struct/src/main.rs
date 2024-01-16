fn main() {
   let s1 = Student{
       name: String::from("xiaoming"),
       age: 18,
       score: 100,
   };

    let s2 = Student::new("xiaoming", 18, 100);

    println!("s1 = {}", s1.name);
    println!("s2 = {}", s2.name);
    println!("s2 = {}", s2.get_name());
    println!("s2 = {}", s2.age);
    println!("s2 = {}", s2.score);
}


struct Student {
    name: String,
    age: i32,
    score: i32,
}

impl Student {
    fn new(name: &str, age: i32, score: i32) -> Student {
        Student {
            name: name.to_string(),
            age,
            score,
        }
    }

    fn get_name(&self) ->&str {
        &self.name
    }

}