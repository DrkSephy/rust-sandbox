/*
 * All values in Rust are stack allocated by default. Values can be 
 * `boxed`(allocated in the heap) using the `box` construct. A box, 
 * with type signature Box<T>, is a smart pointer to a heap allocated 
 * value of type `T`. When a box goes out of scope, its destructor is 
 * called, the inner object is destroyed, and the memory in the heap is
 * freed.

 * Boxed values can be dereferenced using the `*` operator, this removes
 * one layer of indirection. Alternatively, the `let box x = y` 
 * pattern can be used to "unbox" `y` into `x`.
*/

use std::mem;

#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point in the heap, and return a pointer to it
    box Point { x: 0.0, y: 0.0 }
}

fn main() {
    // (all the type annotations are superflouous)
    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 }
    }; 

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = box Rectangle {
        p1: origin(),
        p2: origin()
    };

    // The output of functions can be boxed
    let boxed_point: Box<Point> = box origin();

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = box boxed_origin();

    println!("Point occupies {} bytes  in the stack",
             mem::size_of_val(&point));

    println!("Rectangle occupies {} bytes in the stack",
             mem::size_of_val(&rectangle));

    // box size = pointer size
    println!("Boxed point occupies {} bytes in the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes in the stack",
             mem::size_of_val(&box_in_a_box));

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack",
             mem::size_of_val(&unboxed_point));

    // Unboxing via a destructuring pattern
    let box another_unboxed_point = boxed_point;
    println!("Another unboxed point occupies {} bytes in the stack",
             mem::size_of_val(&another_unboxed_point));
}

