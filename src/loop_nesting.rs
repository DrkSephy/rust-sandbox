/* It is possible to `break` or `continue` outer loops when dealing with
 * nested loops. In these cases, the loops must be annoted with some `label`,
 * and this label is passed to the `break/continue` statement.
*/

fn main(){
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            // This would break only the inner loop
            // break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

}
