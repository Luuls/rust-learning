fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // Cannot read s1 since it is now invalid because s2 took ownership (move) of its resource.
    println!("{s1}");
}
