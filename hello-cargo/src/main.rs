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

    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';
    
    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);

    // tuple of length 3
    let tuple: (i32, &str, char) = (50, "hello", 'c');

    println!("tuple is {:?}", tuple);

    println!("tuple is {:#?}", tuple);


    // classic struct with named fields

    struct Person {
        name: String,
        age: u8,
        is_student: bool
    }

    // tuple struct

    struct Grades(char, char, char, char, f32);

    // unit struct

    struct Unit;

    // instantiate classic struct

    let person = Person {
        name: String::from("Bob"),
        age: 25,
        is_student: true
    };

    // instantiate tuple struct

    let grades = Grades('A', 'A', 'A', 'A', 4.0);

    println!("{} is {} years old. They are a student: {}", person.name, person.age, person.is_student);
    println!("Their grades are: {}, {}, {}, {}, {}", grades.0, grades.1, grades.2, grades.3, grades.4);


    // enum

    enum GameAction {
        Kill,
        Exit,
        MOVE {direction: Direction, speed: u8},
        JUMP,
        RUN,
        KEYS(String, char),   
    }

    enum Direction {
        Up,
        Down,
        Left,
        Right
    }


    // instantiate enum

    let user_action = GameAction::MOVE {direction: Direction::Up, speed: 10};

    let user_action_2 = GameAction::KEYS(String::from("Space"), ' ');

    println!("user_action is {:?}", user_action);
    println!("user_action_2 is {:?}", user_action_2);


    // functions here

    say_hello();

    say_hello_to("Raman Sharma");

    let x = 5;

    let x_plus_one = add_one(x);

    println!("{} + 1 = {}", x, x_plus_one);

    
    

}

fn say_hello() {
    println!("Hello!");
}

fn say_hello_to(name: &str) {
    println!("Hello {}!", name);
}

fn add_one(x: i32) -> i32 {
    x + 1
}
