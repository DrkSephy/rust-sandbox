/*
 * Rust provides no implicit type conversion (coercion) between primitive types.
 * We can explicitly type convert through casting using the `as` keyword.
*/

fn main(){
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
}
