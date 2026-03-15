fn main() {
    // A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.
    // Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.

    struct User {
        id: String,
        username: String,
        active: bool,
        email: String,
    }

    let mut user1 = User {
        id: String::from("1223"),
        username: String::from("Shubham"),
        active: true,
        email: String::from("shubham@yahoo.in"),
    };
    println!("user1 id: {}", user1.id);
    println!("user1 username: {}", user1.username);
    println!("user1 is active: {}", user1.active);
    println!("user1 mail: {}", user1.email);

    user1.email = String::from("shubham@gmail.com");
    println!("user1 mail: {}", user1.email);

    let user2 = User {
        id: String::from("1234"),
        username: user1.username.clone(),
        active: user1.active,
        email: String::from("ryan@gmail.in"),
    };

    println!("user1 username: {}", user1.username); //can't do this cause username was moved. correcting it with clone instead
    println!("user1 is active: {}", user1.active);

    // Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.
    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
    struct Point(i32, i32, i32);
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;
    println!("{x}");
    println!("{y}");
    println!("{z}");

    // You can also define structs that don’t have any fields! These are called unit-like structs
    struct AlwaysEqual;
    let mut subject = AlwaysEqual;

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    // Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug.
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1); // We do not want dbg to take ownership
    rect1.height = 39;
    dbg!(&rect1);

    // Adding methods
    // Methods are similar to functions: We declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object
    // Their first parameter is always self which represents that instance of the struct

    #[derive(Debug)]
    struct Square {
        side: i32,
    }

    impl Square {
        fn about(&self) {
            println!("{self:#?}");
        }

        fn area(&self) -> i32 {
            self.side * self.side
        }
    }

    let square = Square { side: 2 };
    square.about();
    let area = square.area();
    println!("the area is: {area}");

    // In C and C++, two different operators are used for calling methods: You use . if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to (*object).something()

    // Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust with this behavior.

    // Methods use self 'cause they are associated with the type. But there can be functions not assoicated with.
    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    // These can be called as follows:
    let mut rect2 = Rectangle {
        width: 20,
        height: 10,
    };
    let sq = Rectangle::square(rect2.width);
    dbg!(&sq);

    rect2.width = 39;

    dbg!(&rect2);
}
