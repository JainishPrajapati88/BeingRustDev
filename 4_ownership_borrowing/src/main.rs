fn main() {
    //There are two things in memory
    // 1. Stack
    // 2. Heap

    //Heap is the memory part where data is stored and in retrn of that
    //it returns the pointer to the location.

    //And that pointer location is stored in stack
    //last in first out

    //Memory read-write is faster in stack as compared to heap(bcz it uses pointers whereas stack uses the kind of container where data is stored sequentially and follows last in first out so the it doesn't have to deal with pointers)

    //Other languages have garbage collectors which are responsible to handle the unused memory , but rust gives control to us, ownership are set of rules that governs how rust manages the memory

    //Ownership rules :
        // 1. Every value in rust have an owner
        // 2. There can be only one owner of a value at a time
        // 3. When owner gets out of the Scope , value will be dropped


    //Variable Scopes
        // 1. Exterior Scope (the main function in rust) 
        // 2. Interior Scope (example is given below)
    
    //Scope can be declared with {}
    { //Scope starts
        // This variable is limited to this scope
        let greet = String::from("Hello Universe!");
        println!("{greet}");
    }//Scope ended rust will call drop fn to clean memory

    //We can't access greet here , we'll get error at below line
    // println!("{greet}");

    let str = String::from("This is text");
    // let str2 = str;
    // So after coping the str int str2 we can't access str 
    // We must use : let str2 = str.clone();

    takes_ownership(str); // here we passed str to takes_ownership() that means after that we can't use str , bcz strings don't have exact size at compile time

    // println!("{str}");

    let num:i64 = 50;
    make_copy(num);
    //But here we can print num bcz its copied into cp_num while calling fn
    // unlike strings int have known size at compile time in Rust.
    println!("{num}");
}

fn takes_ownership(tk_str:String){
    println!("{tk_str}");
}// here tk_str drops

fn make_copy(cp_num:i64){
    println!("{cp_num}");
}// here cp_num drops