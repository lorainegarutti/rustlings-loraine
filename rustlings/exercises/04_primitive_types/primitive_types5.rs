fn main() {
    let cat: (&'static str, f64) = ("Furry McFurson", 3.5);

    let (name, age) = cat;

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;

    println!("{name} is {age} years old");
}
