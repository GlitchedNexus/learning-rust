// Tuples group together values of different types
// Max 12 elements

pub fn run() {

    let person: (&str, &str, &str) = ("Glitched", "Human", "Somewhere");
    println!("{} is {} and lives {}", person.0, person.1, person.2);

}