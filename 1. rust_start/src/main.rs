fn main() { // this is exterior scope
    // Single line comment
    /*
    This
    is
    Multiline
    Comment*/
    let greet = "Hello, Universe!!!";
    println!("{greet}");

    //By default variables in rust are immutable, so we can't change the value of it after intilization.
    // To make variable mutable, we write mut keyword before variable name...
    let x = 10;
    println!("x is: {}",x);

    //Name shadowing by creating scope
    {// this is interior scope

        //Here i did't have assigned any number to x like below , but for now it will do x = 10 + 99
        // It will consider the value of x from exterior scope so x = 10 + 99
        // We can access the variable declared in exterior scope into interior scope.
        let x = x + 99;
        println!("x is: {}",x);

        //This var x is limitade to this scope , just similar to the variables created in particlar function.
        let x = x - 99;
        println!("x is: {}",x);
    }

    //x = 20; //---> It Will not work until you use mut keyword at declaring variable
    //println!("x is: {}",x);

    let x = 30; //---> But it will work without mut keyword , it kinda overriding the value of x OR Redefining the value of x.
    println!("x is: {}",x);

    //So we reinitilized or recreated the variable x , so that we can change the datatype.
    // When we use "let" keyword rust will Redeclare the variable
    let x = "Reinitilized with different data type";
    println!("{x}");

    //But if variable is declared as mutable , then we can't change the datatype.
    // If the value of variable is not going to change then don't make that mutable otherwise it will give warning...
    //let mut z = 88;
    let z = 88;
    println!("z is: {}",z);

    //z = "Hii"; // this will give error for chaning the datatype...
    //print!("z is: {}",z);

    {
        // I Simply declared y and assigned with 10 in interior scope
        //let y = 10;
        // hence i have not reading this variable so , don't declare it unless you will get warning...
        // So i have comment downed the variable
    }

    // Now i am accessing the value of y declared in interior scope
    //But it will now work, the variable y will not work outside of the scope.
    // The variable declared in interior scope can't be accessed from outside like from exterior scope
    //let y = y + 10;
    //println!("y is: ",{y});


    //Constants
    //The name of const variable must be in upper case and datatype must be defined
    const CONSTANT_VAR:u32 = 500;
    println!("CONSTANT_VAR is: {}",CONSTANT_VAR);

    //Unlike immutable variable , we can't change the value of constant , below code is commented...
    //const CONSTANT_VAR:u32 = 500;
    //println!("CONSTANT_VAR is: {}",CONSTANT_VAR);

}