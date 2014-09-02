fn main(){
    let number: int = 9001;

    println!("Tell me about {}", number);
    
    match number {
        // Match single value
        1 => println!("One!"),

        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("Prime number"),

        // Match an inclusive range
        13..19 => println!("A teen"),

        // Handle rest of cases
        _ => println!("No match found"),
    }

    let boolean = true;

    // Match is also an expression 
    let binary: int = match boolean {
        // The arms of a match must cover all possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

