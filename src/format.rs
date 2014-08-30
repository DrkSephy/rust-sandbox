fn main(){
    //`print!` does not add newlines at end
    print!("January has ");

    // `{ }` are placeholders for arguments that will be stringified
    // `i` suffix means that this literal has type: signed pointer size
    // integer. 
    println!("{} days", 31i);
    
    // Reuse positional arguments with a template
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject}{verb}{predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    // Special formatting can be specified in the placeholder after a `:`
    println!("{} of {:t} people know binary, the other half don't", 1i, 2i);

    // Display an error when missing arguments
    // println!("My name is {0}, {1} {0}", "Bond");
    
}
