fn main(){
    let an_integer = 1u;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {}", unit);

    // Rust compile will warn about unused variables.
    // To supress warnings, prefix variable with underscore
    let _unused_variable = 3u;
    let noisy_unused_variable = 2u;
}
