/*
 * In Rust, almost every statement is an expression, meaning that it 
 * returns a value. We can suppress the output by ending the expression
 * with a semicolon.
*/

fn main(){
    let x = 5u;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x
    };

    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);

}
