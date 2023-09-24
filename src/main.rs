        //All code here is me trying to learn Rust using the Rust by Example document
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

    //Make spaces before or after the argument
    println!("\n\nFormat");
    //Five spaces before the number
    println!("{number:>5} spaces before the number", number=2);
    //Five spaces after the number
    println!("{number:<5} spaces after the number", number=2);
    //five zeros before the number
    println!("{number:0>5} zeros before the number", number=2);
    //five zeros after the number
    println!("{number:0<5} zeros after the number", number=2);
    //five zeros before the number using named argument
    println!("{number:0>width$} zeros before the number using named argument to change the width", number=2, width=5);

    //Enable dead code (code that isn't used anywhere)
    //#[allow(dead_code)]
    //struct Structure(i32);

    //Interesting things or tests
    //println!("\n\nTests");
    //println!("This struct `{}` won't print...", Structure(3));

    //five zeros befero the number, but with variables
    println!("\n\nfive zeros using variables");
    let number: f64 = 2.0;
    let width: usize = 4;
    println!("{number:>width$}");
}
