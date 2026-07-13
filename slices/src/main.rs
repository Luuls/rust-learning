fn main() {
    println!("Hello, world!");

    let a = "ola vai tomando";

    let mut found = false;
    let mut start: usize = 0;
    let mut vai: &str = "";

    // this code contains bug because its not unicode-safe.
    // It also doesn't consider the word if the loop ends and doesn't find a space
    for (i, &letter) in a.as_bytes().iter().enumerate() {
        if letter == b'v' {
            found = true;
            start = i;
        }
        if found {
            if letter == b' ' {
                vai = &a[start..i];
                break;
            }
        }
    };

    let vai = vai;

    println!("The value of vai is: {vai}");

}
