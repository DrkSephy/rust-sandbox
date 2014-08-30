/*
 * Rust provides type safety using its static type checker. Variables can be 
 * type annotated when declared. However, the compiler can infer the type from
 * the context, making annotation not mandatory.
*/
fn main(){
    // Type annotated variable
    let a_float: f64 = 1.0;

    // This variable is an `int`
    let mut an_integer = 5i;

    // Error, The type of a variable cannot be changed
    // an_integer = true;
}

/* Primitive types in Rust:
 * signed int: i8, i16, i32, i65, int
 * unsigned int: u8, u16, u32, u64, uint
 * floating point: f32, f64
 * char: `a`
 * bool either true/false
 * unit type, value: ()
*/
