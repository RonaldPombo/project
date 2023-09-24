fn main() {
    //basic
    println!("The start");
    println!("Hello, world!");
    // learning arguments
    println!("\n\nArgument");
    println!("{0} this is {1}, and {1} this is {0}", "Carlos", "Pedro");
    //learning named arguments
    println!("\n\nNamed Argument");
    println!("{start}, {middle}, {end}",
    start = "This is the start of the prinln using named arguments",
    middle = "This is the second named argument",
    end = "I think this will be useful for when writing things that vary based on multiple user inputs or choices, will see later if its that.");

    //Transform a argument into another format
    println!("\n\nFormat");
    println!("Base 10:               {}", 69);
    println!("Base 2 (binary):       {:b}", 69);
    println!("Base 8 (octal):        {:o}", 69);
    println!("Base 16 (hexadecimal): {:x}", 69);
    println!("Base 16 (hexadecimal): {:X}", 69);
}
