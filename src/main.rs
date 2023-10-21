fn main() {
    let mut x = 5; 
    println!("The value of x is: {x}"); 
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    
    println!("The value of y is: {y}");

    let spaces = "   ";
    println!("The string of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");
    /* // wont compile bc of mut keyword, has to be let and then shadowed to change data type
    let mut spaces2 = "   ";
    spaces2 = spaces2.len();
    //this will compile with two warnings about not needing mut keyword
    let mut spaces3 = "   ";
    let spaces3 = spaces3.len();
    println!("The number of spaces is: {spaces3}");
    */
}
