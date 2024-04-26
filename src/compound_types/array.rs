/*
*   Example Array function. Unlike a tuple, every element of an array must
*   have the same type. In Rust, arrays are special, and unlike arrays in
*   some other languages as they must have a fixed length.
*/

fn main() {
    let array = [1, 2, 3, 4, 5];
}

// Arrays are useful when you want your data allocated on the stack rather
// than the heap (This is discussed further in Chapter 4 - Ownership) or
// when you want to ensure you always have a fixed number of elements.

fn months() {
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
}