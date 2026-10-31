fn main() {
    /*
     * Use: I have a variable that is an arbitrary data type, and I want to convert it to another,
     *      but store it in the same variable, I can use variable shadowing, which allows me to use
     *      the same variable name, but the previous declaration is now invalid,
     */

    //Original declaration = Ignored
    let _corpse_starch_calories = "892.78";
    println!("Oldest Value: {_corpse_starch_calories} = Ignored");

    //Imagine I converted the original string to a numeric value

    // New value of variable after Original
    let _corpse_starch_calories = 892.78;
    println!("2nd Oldest Value: {_corpse_starch_calories} = Ignored");

    // Current value of variable after New value
    let _corpse_starch_calories = 893;
    println!("Current Value: {_corpse_starch_calories} = NOT IGNORED");
}
