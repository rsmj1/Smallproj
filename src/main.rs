//Import / Compile statements:
mod funcs;


/*
    1: You can only have one mutable reference to a variable in scope at a time!! This is why we need to borrow the reference
    2: For a function to modify a current mutable ref, it needs ": &mut" - this means that it is borrowing a mutable reference. The function is then also called with &mut, to indicate that we are allowing it to be borrowed.  
*/


fn main() {
    println!("Hello, new project!");

    let immuta = 12;
    let mut muta: i32 = 10;

    muta = muta -1;
    
    let muta2 = &mut muta;

    *muta2 = *muta2 + 100;
    println!("muta:{}, muta2", muta);


    // if muta < 11 {
    //     println!("1");
    // } else if immuta > 12 {
    //     println!("2");
    // } else{
    //     println!("3");
    // }
    funcs::func1::add10(&mut muta);
    funcs::func1::add(immuta, muta);

    println!("muta: {}, immuta: {}", muta, immuta);
}
