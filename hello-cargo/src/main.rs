fn main() {
    println!("Hello, world!");

    let a_number;

    let a_word = "Ten!";

    a_number = 10;

    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);

    let mut a_number_2 = 10; 
    println!("The number is {}.", a_number_2);

    // Change the value of an immutable variable
    a_number_2 = 15;
    println!("Now the number is {}.", a_number_2);


    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num" 
    let shadow_num = shadow_num + 5; 

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2; 

    println!("The number is {}.", shadow_num);

    let number: u32 = 14;
    println!("The number is {}.", number);

    // Addition, Subtraction, and Multiplication
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);


    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);   // false

    let uppercase_s = 'S';
    let lowercase_f = 'f';
    let smiley_face = '😃';

    println!("{} {} {}", uppercase_s, lowercase_f, smiley_face);

}
