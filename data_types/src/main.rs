fn main() {
    // println!("Hello, world!");
    println!("===========================================");
    const HOURS_IN_A_WEEK: u32 = 24 * 7;
    println!("hours in a week: {HOURS_IN_A_WEEK}");
    println!("hours in a week: {}", HOURS_IN_A_WEEK);
    println!("hours in a week: {}", 24 * 7);
    fn hours_in_a_week() -> u32 {
        return 24 * 7;
    }
    println!("hours in a week: {}", hours_in_a_week());
    println!("===========================================");

    let number: u32 = 123;
    {
        let number = "Three";
        println!("Shadowed number: {number}");
    }
    println!("Original number: {number}");

    println!("===========================================");

    // Scalar Types -> They represent a single value
    // Has four primary scalar types -> integers, floating-type, Booleans, characters
    // igned and unsigned refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned).

    // let number: u32 = -1; // Gives error
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Hi!");
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; //[3,3,3,3,3]

    let _first = a[0];

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let condition = true;
    let number = if condition { 5 } else { 6 };


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }


}
