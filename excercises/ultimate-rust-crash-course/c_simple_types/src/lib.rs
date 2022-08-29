// 5.  What a mess -- functions in a binary! Let's get organized!
//
// - Make a library file (src/lib.rs)
// - Move all the functions (except main) into the library
// - Make all the functions public with `pub`
// - Bring all the functions into scope using use statements. Remember, the name of the library
//   is defined in Cargo.toml.  You'll need to know that to `use` it.
//
// `cargo run` should produce the same output, only now the code is more organized. 🎉

pub fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

pub fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

pub fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

pub fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}

pub fn print_distance_original(z: (f32, f32)) {
    // Using z.0 and z.1 is not nearly as nice as using x and y.  Lucky for
    // us, Rust supports destructuring function arguments.  Try replacing "z" in
    // the parameter list above with "(x, y)" and then adjust the function
    // body to use x and y.
    println!(
        "Distance to the origin is {}",
        (z.0.powf(2.0) + z.1.powf(2.0)).sqrt());
}

    // 6. SOLUTION:
pub fn print_distance(coords: (f32, f32)) {

    let (x, y) = coords;

    println!(
        "Distance to the origin is {}",
        ( x.powf(2.0) + y.powf(2.0) ).sqrt());
}