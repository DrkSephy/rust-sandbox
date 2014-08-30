fn main(){
    // Variables are immutable by default. 
    // We can override this behavior using `mut` modifier. 

    // immutable 
    let _immutable_variable = 1i;
    // mutable
    let mut mutable_variable = 1i;

    println!("Before mutation: {}", mutable_variable);

    mutable_variable += 1;
    println!("After mutation: {}", mutable_variable);

    // Below will fail
    // _immutable_variable += 1;


}
