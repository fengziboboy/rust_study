fn main() { 
    println!("Hello, world!"); 
 
    another_function(); 
    another_function2(6); 
    another_function3(1, 2);

    let x = five();
    println!("The value of y is: {}", x); 

    let x = plus_one(5); 
    println!("The value of x is: {}", x); 
} 
 
fn another_function() { 
    println!("Another function."); 
} 

// 不能重载呀
fn another_function2(x: i32) {
    println!("The value of number is:{}", x);
}


// 多个参数 
fn another_function3(x: i32, y: i32) { 
    println!("The value of x is: {}", x); 
    println!("The value of y is: {}", y); 
}  


// 在以上的five函数中，除了数字5，没有分号，没有任何其他的函数调用、宏调用，甚至是let语句，
// 但它在Rust中确实是一个有效的函数。
// 注意，这个函数的返回值类型通过-> i32被指定了。
fn five() -> i32 {
    5
}


// 假如我们给函数plus_one结尾处的x + 1加上分号（如下所示），那么这个表达式就会变为语句并进而导致编译时错误
// 这里的错误提示信息“mismatched types”（类型不匹配）揭示了上面代码中的核心问题。
// 我们在定义plus_one的过程中声明它会返回一个i32类型的值，但由于语句并不会产生值，
// 所以Rust默认返回了一个空元组，也就是上面描述中的()。
fn plus_one(x: i32) -> i32 { 
    x + 1 
} 