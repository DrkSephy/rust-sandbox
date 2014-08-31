fn main(){
    // Local inference
    let elem = 5u8;

    // Create an empty vector
    let mut vec = Vec::new();

    // Compiler currently does not know what type `vec` is. However...
    vec.push(elem);

    // Now the compiler knows that `vec` is conposed of `u8`s. 
    println!("{}", vec);
}
