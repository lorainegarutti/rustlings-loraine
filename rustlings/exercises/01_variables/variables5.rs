fn main() {
    let number: &'static str = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = number.len();
    println!("Number plus two is: {}", number + 2);
}
