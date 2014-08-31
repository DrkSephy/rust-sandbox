/*
 * Branching using if/else is similar to C, but no () needed. Due to Rust
 * type safety, all the branches must return the same type.
*/

fn main(){
    let n = 5i;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            // This expression will return an `int`
            10 * n
        } else {
            println!(", and is a big number, reduce by two");
            // This expression MUST return an `int` as well
            n / 2
            // If we suppress the above expression, an error will be thrown
            // regarding if/else having incompatible types. 
            // Expected `int`, found `()`
    };

    println!("{} -> {}", n, big_n);
}
