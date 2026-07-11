fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let tup = (1, 2.0, 4u64, 'o', "oi");

    // can individually declare as mutable
    let (a, b, mut c, d, e) = tup;

    println!("c before: {c}");
    c = 6;
    println!("c after: {c}");
    // a = 2; -> error

    // tuple indexes with dot
    println!("tup.0: {}", tup.0);
    println!("tup: {tup:?}");

    // arrays are fixed size. Use vec for a growing array.
    let mut months = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];

    // indexes like other languages
    months[0];
    for month in months {
        print!("{month}, ");
    }

    // can assign because array is mutable
    months[0] = "jan1";
    println!("\n{}", months[0]);

}
