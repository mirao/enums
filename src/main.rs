#[derive(Debug)]
enum Gender {
    Man,
    Woman,
}
impl Gender {
    fn print(&self) {
        println!("I'm a {:#?}", self);
    }
}

#[derive(Debug)]
struct Person {
    gender: Gender,
    age: u8,
}

impl Person {
    fn print_me(&self) {
        // Print gender
        self.gender.print();
        // Print age
        println!("Age: {}", self.age);
    }
}
fn main() {
    let mut me = Person {
        gender: Gender::Man,
        age: 30,
    };
    me.print_me();
    me.gender = Gender::Woman;
    me.age = 20;
    me.print_me();
}
