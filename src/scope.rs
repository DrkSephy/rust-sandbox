fn main(){
    // This variable will live in the main function
    let long_lived_variable = 1i;

    // Below is a block, containing a smaller scope than main
    {
        // This variable exists in this block only
        let short_lived_variable = 2i;

        println!("inner short: {}", short_lived_variable);

        // This variable will shadow the outer instance
        let long_lived_variable = 5_f32;

        println!("inner long: {}", long_lived_variable);
    }
    // Below will fail since `short_lived_variable does not exist in this scope"
    // println!("outer short: {}", short_lived_variable);
    println!("outer long: {}", long_lived_variable);

}
