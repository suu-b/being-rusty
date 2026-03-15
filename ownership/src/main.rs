fn main2() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here
    // To keep using s, one cermonial and tedious way could be to let takes ownership return it afteruse.
    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward.
} // Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn main() {
    // Ownership is the set of rules that governs how a rust program manages memory
    // Ownership is way to ensure memory safety
    // Some languages explicitly allow access to memory memory management (Like C) and others have inbuilt garbage collections & other guardrails like Java

    // Rust uses a third approach:
    // Memory is managed through a system of ownership with a set of rules that compiler checks at the compiletime

    // Rules:
    // 1. Each value in rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // s is string literal. stored in stack.
    // s is not valid here
    {
        let s = "string"; // s is valid from here
        println!("here: {s}");
    }
    // s is not valid here. scope ends.

    // is mutable. therefore, stored on the heap. the memory is requested fron the memory allocator at the runtime
    let mut s = String::from("hello");
    s.push_str(" world!");
    println!("{}", s);

    // Given the heap memory allocated, it is important when to deallocate it and how
    // In languages like Java, a GC takes care of this
    // In other languages like C, it is upon the shoulders of the dev to pair every alloc with a free at the correct time to ensure no memory is wasted/ there is no invalid variable/ we do not deallocate twice causing a bug

    // Rust takes a different path, the memory is deallocated at its own once the variable exits the scope. A special function called drop() is called beneath.
    // This is similar to RAII -- Resource Acquisition in Initialization pattern in C++

    // Here, first num is created on the stack
    // The value of num is copied to create num1 on the stack
    let num: u32 = 32;
    let num1 = num;

    // Here, it is a bit different
    // First in stack metadata block of s1 is formed (pointer, length, capacity).
    // The pointer points to the memory address in the heap where the actual string data is stored
    // When s2 points to s1 in the next line, the metadata is copied to a new stack location but it keeps on pointing to the same underlying heap location.
    // This is called shallow copying. Deep copying is expensive.
    let mut s1 = String::from("Hello");
    let mut s2 = s1;

    // When both go out of scope in such case, it would be an issue as same heap memory would be attempted to be freed twice. Therefore, as soon as s2 is declared, s1 is considered no longer valid. This kinda shallow copying in rust is called a move -- s1 was moved to s2;
    // println!("{s1} world!"); //throws an error

    // When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately.
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    // Deep copying:
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // Copy trait
    // If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable. Any type implementing Drop cannot implement Copy

    main2();

    // MOVING basically means transfering of ownership of a chunk of heap from one variable/scope to another.
    // But that returning thing is too ceremonial.

    // ========================================================================
    // ========================================================================
    // We can instead use REFERENCE
    // Reference is like a pointer in that it's an address we can follow to access the data stored at that address; that data is owned by some other variable

    fn calc_length(s: &String) -> usize {
        s.len()
    }
    let mut s1 = String::from("Hello world!");
    let len = calc_length(&s1);
    println!("The length is {len}");

    // &s1 syntax lets us create a reference that refers to the value of s1. Because the reference does not own it, the value it points to will not be dropped when the reference stops being used.
    // These ampersands represent references, and they allow you to refer to some value without taking ownership of it.
    // in the function, s becomes a new metadata block in the stack memory that points to the s1 block that in turn points to the string.
    // this is called BORRROWING
    // We cannot mutate a regular reference, we need to make it a mutable reference
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // If we have a mutable reference to a value, we can have no other references to that value
    // LIMITATION: If we have one mutable reference, we cannot have other references to the value
    // This prevents race condition.

    // Dangling Pointers
    // A dangling pointer is a pointer that references a location in memory that may have been given to someone else -- by freeing some memory while preserving a pointer to that memory

    // Throws an error because
    // fn dangle() -> &String { //returns a reference to a string
    //     let mut s = String::from("Hello"); // s is a new string
    //     &s //we return a reference to s
    // }// here s is dropped cause out of scopped, how can we keep a pointer to it when it is not there!!

    // let ref = dangle();

    // rules of reference:
    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.

    // ========================================================================
    // ========================================================================
    // SLICE
    // Slices let you reference a contiguous sequence of elements in a collection. A slice is a kind of reference, so it does not have ownership.

    // Write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    // A string slice is a reference to a contiguous sequence of the elements of a String
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
