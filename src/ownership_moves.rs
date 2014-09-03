/*
 * Variables in rust are in charge of freeing their resources. Also,
 * resources can only have ONE owner, otherwise resources would get
 * freed more than once. 

 * When doing assignments: `let x = y`, or passing function arguments 
 * by value foo(x), the *ownership* of the resources, if any, is 
 * transferred, known as a "move" in Rust.

 * After moving resources, the previous owner can no longer be used. This
 * avoids the creation of *dangling pointers*. 
*/

// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<int>) {
    println!("Destroying a box that contains {}", c);

    // `c` will be destroyed in this scope, and the memory will be 
    // freed as well.
}

fn main() {
    // Stack allocated integer
    let x = 5u;

    // "Copy" `x` into `y`, there are no resources to move.
    let y = x;

    // Both values can be independently used.
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a heap allocated integer
    let a = box 5;

    println!("A contains: {}", a);

    // "Move" `a` into `b`
    // Pointer `a` gets copied (*not* the data on the heap, just its
    // address) into `b`. Now both are pointers to the *same* heap 
    // allocated data. But now, `b` *owns* the heap allocated data;
    // `b` is now in charge of freeing the memory in the heap.
    let b = a;

    // After the previous move, `a` can no longer be used
    // Error! `a` can no longer access the data, because it no longer
    // owns the heap memory.
    // println!("A contains: {}", a);

    // "Move" `b` into the funcion; `b` gives up ownership of the 
    // heap data
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action
    // would result in dereferencing freed memory, but it is forbidden
    // by the compiler.
    // Error! Same reason as previous Error
    // println!("b contains: {}", b);
}

