fn main(){
    /*
     * Integers can be expressed using hexadecimal, octal or binary notation.
     * Hexadecimal: 0x, Octal: 0o, Binary: 0b
     * `u` suffix to indicate literal is an unsigned int
     * `i` suffix to indicate literal is a signed int
    */
    println!("1 + 2 = {}", 1u + 2);
    
    // integer subtraction
    println!("1 - 2 = {}", 1i - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04t}", 0b0011u & 0b0101);
    println!("1 << 5 is {}", 1u << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u >>2);


}
