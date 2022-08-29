// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    // 1. Try running this code with `cargo run` and take a look at the error.
    //
    // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
    // doing `cargo run` should succeed and print something out.

    // ORIGINAL CODE;
    // {
    //     let area = area_of(width, height);
    // }
    // println!("Area is {}", area);


    // 1. SOLUTION;
    // The error is caused by scoping; The 'area' variable is declared inside the brackets and will
    // then be unavailable once that scope ends and we call the println macro.
    // Three possible solutions to this:
    // 1. Move the println! macro inside the brackets putting the variable and the print macro
    //    inside the same scope
    // 2. Declare a mutable 'area' variable in the root scope, initializing it with a type and a value
    // 3. Just get rid of that useless inner scope. Its not needed or really doing anything:

    let area = area_of(width, height);

    println!("Area is {}", area);

    // 2. The area that was calculated is not correct! Go fix the area_of() function below, then run
    //    the code again and make sure it worked (you should get an area of 28).

    // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
    //    Create the `volume` function!  It should:
    //    - Take three arguments of type i32
    //    - Multiply the three arguments together
    //    - Return the result (which should be 280 when you run the program).
    //
    // If you get stuck, remember that this is *very* similar to what `area_of` does.
    //
    println!("Volume is {}", volume(width, height, depth));

    // 3. SOLUTION:

}

fn area_of(x: i32, y: i32) -> i32 {
    // 2a. Fix this function to correctly compute the area of a rectangle given
    // dimensions x and y by multiplying x and y and returning the result.
    //

    // Original code:
    // return 0;
    // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
    //            `return` on the last line of a function. Change the last line to be a
    //            "tail expression" that returns a value without using `return`.
    //            Hint: `cargo clippy` will warn you about this exact thing.

    // 2. SOLUTION:
    // The math of an area is well known. But we dont need the return here, so lets just make it a
    // tail expression
    x * y
}

// 3. SOLUTION:
// Pretty straight forward; multiply all three as a tail expression. I am also making sure we return
// an absolute value here as negative volume is not really a thing

fn volume(x: i32, y: i32, z: i32) -> i32 {
    (x * y * z).abs()
}
