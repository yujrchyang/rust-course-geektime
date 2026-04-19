fn main() {
    test_animal();
    test_human();
}

trait Human {
    fn speak(&self);
}

struct Man;
struct Woman;

enum Gender {
    Man(Man),
    Woman(Woman),
}

impl Gender {
    fn new(g: i32) -> Self {
        if g == 1 {
            Gender::Man(Man)
        } else {
            Gender::Woman(Woman)
        }
    }

    fn speak(&self) {
        match self {
            Gender::Man(m) => m.speak(),
            Gender::Woman(w) => w.speak(),
        }
    }
}

impl Human for Man {
    fn speak(&self) {
        println!("man");
    }
}

impl Human for Woman {
    fn speak(&self) {
        println!("woman");
    }
}

fn creawe_human(g: i32) -> Gender {
    Gender::new(g)
}

fn test_human() {
    let man = creawe_human(1);
    let woman = creawe_human(2);
    man.speak();
    woman.speak();

    let genders = vec![creawe_human(1), creawe_human(2)];
    for g in genders {
        g.speak();
    }
}

trait Animal {
    fn speak(&self);
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn speak(&self) {
        println!("cat");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("dog");
    }
}

fn animal_speak_static(animal: impl Animal) {
    animal.speak();
}

fn animal_speak_dyn(animals: Vec<Box<dyn Animal>>) {
    for a in animals {
        a.speak();
    }
}

fn animal_create(typ: i32) -> Box<dyn Animal> {
    if typ == 1 {
        Box::new(Cat)
    } else {
        Box::new(Dog)
    }
}

fn test_animal() {
    let cat = Cat;
    let dog = Dog;
    animal_speak_static(cat);
    animal_speak_static(dog);

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
    animal_speak_dyn(animals);

    let animals_create = vec![animal_create(1), animal_create(2)];
    animal_speak_dyn(animals_create);
}
