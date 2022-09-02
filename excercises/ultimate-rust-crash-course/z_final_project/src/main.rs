// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        // "blur" => {
        //     if args.len() != 2 {
        //         print_usage_and_exit();
        //     }
        //     let infile = args.remove(0);
        //     let outfile = args.remove(0);
        //     // **OPTION**
        //     // Improve the blur implementation -- see the blur() function below
        //     blur(infile, outfile);
        // }

        // SOLUTION:
        // Improving blur() by adding an optional arg to specify blur amount
        "blur" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            // Get an optional 3rd arg for the blur amount. Default to 2.0 if missing
            let blur_amount = match args.remove(0).parse::<f32>() {
                Ok(x) => x,
                _ => 2.0,
            };

            blur(infile, outfile, blur_amount);
        }

        // **OPTION**
        // Brighten -- see the brighten() function below

        // SOLUTION:
        "brighten" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            let brighten_amount = match args.remove(0).parse::<i32>() {
                Ok(x) => x,
                _ => 0,
            };

            brighten(infile, outfile, brighten_amount);
        }

        // **OPTION**
        // Crop -- see the crop() function below

        // SOLUTION:
        "crop" => {
            if args.len() < 6 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            // Get an optional 3rd arg for the blur amount. Default to 2.0 if missing
            let x = match args.remove(0).parse::<u32>() {
                Ok(x) => x,
                _ => 0,
            };

            let y = match args.remove(0).parse::<u32>() {
                Ok(x) => x,
                _ => 0,
            };

            let width = match args.remove(0).parse::<u32>() {
                Ok(x) => x,
                _ => 0,
            };

            let height = match args.remove(0).parse::<u32>() {
                Ok(x) => x,
                _ => 0,
            };

            crop(infile, outfile, x, y, width, height);
        }

        // **OPTION**
        // Rotate -- see the rotate() function below

        // SOLUTION:
        "rotate" => {
            if args.len() < 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            let rotate_amount = match args.remove(0).parse::<i32>() {
                Ok(x) => x,
                _ => 0,
            };

            // Check if the value we got is allowed
            if ![90, 180, 270].contains(&rotate_amount) {
                println!("\nERROR: Please rotate by on of the following values 90, 180 or 270");
                print_usage_and_exit();
            }

            rotate(infile, outfile, rotate_amount);
        }

        // **OPTION**
        // Invert -- see the invert() function below

        // SOLUTION:
        "invert" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            invert(infile, outfile);
        }

        // **OPTION**
        // Grayscale -- see the grayscale() function below

        // SOLUTION:
        "grayscale" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            grayscale(infile, outfile);
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() < 1 {
                print_usage_and_exit();
            }

            let outfile = args.remove(0);
            fractal(outfile);
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

        // SOLUTION:
        "generate" => {
            if args.len() < 1 {
                print_usage_and_exit();
            }

            let outfile = args.remove(0);
            generate(outfile);
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("\nUSAGE (when in doubt, use a .png extension on your filenames):");
    println!("--------------------------------------------------------------");
    println!("blur INFILE OUTFILE AMOUNT");
    println!("brighten INFILE OUTFILE AMOUNT");
    println!("crop INFILE OUTFILE X_START Y_START WIDTH HEIGHT");
    println!("rotate INFILE OUTFILE AMOUNT [90, 180 or 270]");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

// fn blur(infile: String, outfile: String) {
//     // Here's how you open an existing image file
//     let img = image::open(infile).expect("Failed to open INFILE.");
//     // **OPTION**
//     // Parse the blur amount (an f32) from the command-line and pass it through
//     // to this function, instead of hard-coding it to 2.0.
//     let img2 = img.blur(2.0);
//     // Here's how you save an image to a file.
//     img2.save(outfile).expect("Failed writing OUTFILE.");
// }

fn blur(infile: String, outfile: String, amount: f32) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    img.blur(amount)
        .save(outfile)
        .expect("Failed writing OUTFILE.");
}

// fn brighten(infile: String, outfile: String) {
//     // See blur() for an example of how to open / save an image.

//     // .brighten() takes one argument, an i32.  Positive numbers brighten the
//     // image. Negative numbers darken it.  It returns a new image.

//     // Challenge: parse the brightness amount from the command-line and pass it
//     // through to this function.

//     // Here's how you save an image to a file.
//     img2.save(outfile).expect("Failed writing OUTFILE.");
// }

fn brighten(infile: String, outfile: String, amount: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    img.brighten(amount)
        .save(outfile)
        .expect("Failed writing OUTFILE.");
}

// fn crop(infile: String, outfile: String) {
//     // See blur() for an example of how to open an image.

//     // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
//     // You may hard-code them, if you like.  It returns a new image.

//     // Challenge: parse the four values from the command-line and pass them
//     // through to this function.

//     // See blur() for an example of how to save the image.
// }

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    img.crop_imm(x, y, width, height)
        .save(outfile)
        .expect("Failed writing OUTFILE.");
}

// fn rotate(infile: String, outfile: String) {
//     // See blur() for an example of how to open an image.

//     // There are 3 rotate functions to choose from (all clockwise):
//     //   .rotate90()
//     //   .rotate180()
//     //   .rotate270()
//     // All three methods return a new image.  Pick one and use it!

//     // Challenge: parse the rotation amount from the command-line, pass it
//     // through to this function to select which method to call.

//     // See blur() for an example of how to save the image.
// }

fn rotate(infile: String, outfile: String, amount: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    match amount {
        x if x == 90 => img.rotate90(),
        x if x == 180 => img.rotate180(),
        x if x == 270 => img.rotate270(),
        _ => img,
    }
    .save(outfile)
    .expect("Failed writing OUTFILE.");
}

// fn invert(infile: String, outfile: String) {
//     // See blur() for an example of how to open an image.

//     // .invert() takes no arguments and converts the image in-place, so you
//     // will use the same image to save out to a different file.

//     // See blur() for an example of how to save the image.
// }

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");

    img.invert();

    img.save(outfile).expect("Failed writing OUTFILE.");
}

// fn grayscale(infile: String, outfile: String) {
//     // See blur() for an example of how to open an image.

//     // .grayscale() takes no arguments. It returns a new image.

//     // See blur() for an example of how to save the image.
// }

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    img.grayscale()
        .save(outfile)
        .expect("Failed writing OUTFILE.");
}

// fn generate(outfile: String) {
//     // Create an ImageBuffer -- see fractal() for an example

//     // Iterate over the coordinates and pixels of the image -- see fractal() for an example

//     // Set the image to some solid color. -- see fractal() for an example

//     // Challenge: parse some color data from the command-line, pass it through
//     // to this function to use for the solid color.

//     // Challenge 2: Generate something more interesting!

//     // See blur() for an example of how to save the image
// }

fn generate(outfile: String) {
    let space_x = 20;
    let space_y = 40;

    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let red = 20 as u8;
        let blue = 20 as u8;
        let green = 20 as u8;

        if x % space_x == 0 {
            *pixel = image::Rgb([255 as u8, 0 as u8, 0 as u8])
        } else if y % space_y == 0 {
            *pixel = image::Rgb([255 as u8, 0 as u8, 0 as u8])
        } else if x % space_x == 5 {
            *pixel = image::Rgb([0 as u8, 100 as u8, 0 as u8])
        } else if y % space_y == 5 {
            *pixel = image::Rgb([0 as u8, 100 as u8, 0 as u8])
        } else {
            *pixel = image::Rgb([red, green, blue]);
        }
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
