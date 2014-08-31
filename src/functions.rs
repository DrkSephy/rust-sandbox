// Function which returns a boolean value
fn is_divisible_by(lhs: uint, rhs: uint) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, so the `return` keyword is not necessary.
    lhs % rhs == 0
}

// Functions that do not return a value actually return the unit type `()`
fn fizzbuzz(n: uint) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the signature
fn fizzbuzz_to(n: uint){
    for n in range(1, n + 1){
        fizzbuzz(n);
    }
}

fn main(){
    fizzbuzz_to(100);
}
