#![allow(unused_variables)] //applies directive to the whole file, must be on line 1
/*
 * Type Aliases
 *      benefit: is an alternate name the I can assign to an existing type, think of C,
 *               where after I import the string header file, instead of saying, string varName = value;
 *               I can say str varName = value. Probably not the correct syntax but you get the point :)
 *
 * Format:
 *      In the global section at the top of the file
 *      type Alias_name = type
 *
 * Compiler Directive
 *      An annotation that tells the computer how to parse the source code
 *
 * Format:
 *      Write the directive directly above the line you want to manipulate
 *
 *      #[directive()]
 */

type Legion = i32;
/*
 * Apply directive to both variables at once
 */
//#[allow(unused_variables)]
fn main() {
    /*
     * Apply a compiler directive to my var declaration so it mutes the warning
     */
    //#[allow(unused_variables)]
    let fav_legion: Legion = 9;
    //println!("Favorite Legion: {fav_legion}th");

    let fav_legion: Legion = 1;
    //println!("Favorite Legion: {fav_legion}st");
}
