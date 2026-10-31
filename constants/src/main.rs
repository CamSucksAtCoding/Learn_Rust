const EMPEROR_OF_MANKIND: &str = "Neoth";
fn main() {
    /*
     * Variables that cant be changed or shadowed
     *
     * Difference between immutable and const
     *      - immutables are limited to the functions scope while constants are able to be declared at the file level
     *
     *      - constants need to be known at compile time, so do not use a const to take in user input or anything else that
     *        wont be immediately defined when I complile my code
     *
     * Format:
     *      IMPORTANT: When creating a const variable, I need to explicitly declare its datatype, EX: i32, i64, &str, etc
     *      const VARIABLE_NAME:datatype = value;
     */
    println!("The name of the Emperor is {EMPEROR_OF_MANKIND} ")
}
