/*
 * The `type` statement can be used to give a new name to an existing type.
 * Types must have `CamelCase` names, otherwise the compiler will raise a 
 * warning. The exception to this rule are primitive types: uint, f32, etc.
*/

// 'NanoSecond' is a new name for 'u64'
type NanoSecond = u64;
type Inch = u64;

// Use an attribute to silence compiler warning
#[allow(non_camel_case_types)]
type uint64_t = u64;

fn main(){
    // `NanoSecond` = `Inch` = `uint64_t` = `u64`
    let nanoseconds: NanoSecond = 5 as uint64_t;
    let inches: Inch = 2 as uint64_t;

    // Type aliases do not provide extra type saftey, because aliases
    // are *not* new types.
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);

}
