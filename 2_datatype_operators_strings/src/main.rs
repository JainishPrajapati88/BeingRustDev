fn main() {

    //Integers
    // 2 types : Signed
    //           Unsigned

    // Bydefault variable creation like this creates variable in singned int var of size 32bit
    let a = 10;
    println!("{a}");

    //Modifying the size of signed and unsigned integers
    let age:i64 = 17; // signed int sized 64bits
    let year:u128 = 2024;   // Unsigned int sized 128bits
    println!("My age is {age} and running year is {year}");

    //The size of signed and unsigned int var will be dependent on Based on cpu architecture.
    // When we use isigned & usigned 
    // int var will be sized 32bit on x86 processors
    // int var will be sized 64bit on x64 procesors 
    let marks:isize = 100;
    let percentage:usize = 99;
    println!("Marks : {marks} and percentage : {percentage}");

    //Float
    //here f64 is default
    println!("In Rust to specify the size float variable , we can do that same thing which we were doing for integers!!");
    let rank:f32 = 1.11;
    println!("My DDCET Rank is : {rank}");

    //Boolean
    let you_got_admission:bool = true;
    println!("You got admission? : {you_got_admission}");

    //Character
    let first_name_char:char = 'J'; // can't you double quotes bcz "" - text inside it is considered as string and character must be encloses with ''. 
    println!("Your first name starts from {first_name_char}");

    //String
    let first_name:&str = "Jainish";
    println!("Your first name is {first_name}");

    //String objects
    //string::new()
    //string::from()
    let mut str = String::new(); // empty string object created , we will need push_str() to initilize value
    str.push_str("String initilized");

    let str2 = String::from("initilized at declaration of string");
    //str.len() - to know lenght of string
    println!("{str} and {str2}");

    //type casting
    //float to int is also same as int to float
    let decimal:f32 = 18.18;
    println!("{decimal}");
    println!("after type casting into integer");
    let intval = decimal as u32;
    println!("{intval}");

    // char to ascii means char to int
    // the same way we can do int to char , ascii to char
    let ascii_of_f_char = first_name_char as i32;
    println!("Ascii of {} is {}",first_name_char,ascii_of_f_char);

    // bool to int
    let int_of_bool = you_got_admission as u32;
    // True - 1
    // False - 0
    println!("Integer of {you_got_admission} is {int_of_bool}");

    //Arithmatic operators
    let val1 = 10;
    let mut val2 = 5;
    // the same way (+,-,*,/,%) operations can be performed in Rust
    println!("Sum of {val1} and {val2} is {}",val1+val2);  

    //Assignment operators
    // = , += ,=+ , -=,=- etc.. like same with other arithmatic operations
    val2 += 5;
    println!("Val2 after adding +5 in that : {val2}");  

    //Comparision operators
    // > ,< , >= , <= , != ,==
    // We will implement these in control statements part

    //Logical operator
    // && , || , !
    // Also will implement in control statements part
}
