pub fn run() {

    // Print to console
    println!("Hello from the 'print.rs' file :D");

    // Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Glitched", "Nexus");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}.", 
        "Glitched", "Nexus", "Sleep"
    );

    // Named Arguments
    println!(
        "{name} likes to {activity}.", 
        name="Glitched", 
        activity="Sleep"
    );

    // Placeholder Traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder For Debug Trait
    println!("{:?}", (12, true, "hello"));

    // Basic Expression
    println!("10 + 10 = {}", 10 + 10);
}