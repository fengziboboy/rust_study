fn main() {
    let celsius: f32 = 26.5; 
    println!("Hello, 摄氏温度:{}", celsius);

    let fahrenheit = convert_fahrenheit(celsius);
    println!("摄氏温度:{} 转化为华氏温度：{}", celsius, fahrenheit);

    let celsius2 = convert_celsius(fahrenheit);
    println!("华氏温度:{} 转化为摄氏温度：{}", fahrenheit, celsius2);


    // 斐波那契递归实现。

    for n in 1..10 {
        println!("{}: {}", n, fibonacci(n));
    }
}

fn convert_fahrenheit(x:f32) -> f32 {
    x * 9.0 / 5.0 + 32.0
}

fn convert_celsius(x:f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}


fn fibonacci(n:i32) -> i32 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
