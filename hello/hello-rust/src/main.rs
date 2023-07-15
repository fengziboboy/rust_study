fn main() {
    println!("Hello22, world!");
    const MAX_POINTS: u32 = 100_00;
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}, const:{}", x, MAX_POINTS);

    let spaces = " ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);

    //let mut spaces1 = " ";
    //spaces1 = spaces1.len();

     
    another_function(); 
} 
 
fn another_function() { 
    println!("Another function."); 
} 

