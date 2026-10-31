use std::f32::consts;

fn main() {
    /*
     * Declaring variables
     *
     * format:
     *  let variable_name:i32 = value;
     *
     *
     *
     * Declaring a variable that will be subject to change
     *
     * format:
     *   let mut variabel_name = value
     *
     * Interpolation: placing a variable inside of a string to print out its value in the string
     *  format:
     *      let signature = "Helbrecht";
     *      println!("blah blah blah, we are on an endless crusade, screw the codex.
     *
     *      Best,
     *      {signature}");
     *
     * Immutable variables
     *   format:
     *
     */

    /*Immutable variables*/
    let titus_age = 400;
    println!("Titus is around {titus_age} years old");

    let mut fav_chapter = "Blood Angels";
    println!(
        "My favorite chapter is the {fav_chapter}, but everyone looks like Horus for some reason"
    );
    fav_chapter = "Scythes of the Emperor";
    println!(
        "My favorite chapter that has some insane drip is the {fav_chapter}, look at them in the Astartes II trailer"
    );
}
