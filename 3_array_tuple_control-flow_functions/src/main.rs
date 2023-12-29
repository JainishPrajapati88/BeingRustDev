use std::io; // used for user console input

fn main() {
    println!("Array , tuples , control flow and functions");
    //tuples are enclosed with ()
    let tup = (300,200,100);

    //We can't println tuple directly like we print variables
    println!("{:?}",tup);

    //printing particular element of tuple by index (index starts from 0)

    println!("{}",tup.0);//we are printing 300 which is first element of tuple

    //we can even fix the scalar type of particular element in tuple

    let tup1:(u128 ,f64,char,bool) = (30,30.30,'J',true);
    println!("{:?}",tup1);

    // putting _ just before var name without whitespance never let rust compiler to throw warning
    // when the variable never readed 
    let (_w,_x,_y,_z) = tup1; // We also can assign values of tuple to variables sequentially like 30 -> w , 30.30 -> x and so on...
    println!("Your first name starts from : {}",_y);

    //Arrays
    // enclosed with [] , if we haven't defined size then we can store lot of values in it...
    let arr: [i32; 5] = [10,20,30,40,50]; //we can store only one scalar type data in array, and can set limit into it...
    println!("{:?}",arr);

    //Implementing loop
    let mut index = 0;

    // we should not use this loop when we have to execute code on basis on certain conditions
    // loop{
    //     println!("{index} element is {}",arr[index]);
    //     index += 1;
    // }

    // index = 0;

    //While loop
    while index < 5{
        println!("{index} element is {}", arr[index]);
        index += 1;
    }

    //for loop
    index = 0;
    for element in arr{
        println!("{index} element is {element}");
        index += 1;
    }

    println!("Printing element on basis on inputed index....");

    // temp variable to store imidiate result of console input
    // Rust by default takes input in text format
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to readline");
    
    // To parse string val into int
    let input:usize = input.trim().parse().expect("Entered value is not number");

    println!("The value at index[{input}] is {}",arr[input]);


    // if , if - else and if - elsif ladder is same as other languages...

    //program which takes number fro user input and checks wheather its odd or even
    loop
    {
        let mut num = String::new();

        println!("Enter a number (0 to exit): ");
        io::stdin().read_line(&mut num).expect("Failed to take number");

        let num:usize = num.trim().parse().expect("entered input is not number");
        // Calling function which takes the number to validate 
        odd_or_even(num);

        if num == 0 // Don't even call the function when user wanted to exit..
        {
            break;
        }
        else{
            let res:usize = odd_or_even(num); // Calling the function and getting the returned value on which basis we will println wheather number is odd or even

            if res == 0{
                println!("{num} is even");
            }else{
                println!("{num} is odd");
            }
        }
    }
 
}

fn odd_or_even(val:usize) -> usize // -> usize means function will return an int
{
        if val%2 == 0 {
            0 // in rust we directly write the value which we want to return 
        }else{
            1 // without semi colon ;
        }
}