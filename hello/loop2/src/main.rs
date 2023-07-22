fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    /* while条件循环 */

    // 在每次执行循环体之前都判断一次条件，假如条件为真则执行代码片段，
    // 假如条件为假或在执行过程中碰到break就退出当前循环。

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");


    /*  使用for来循环遍历集合 */ 

    // 先看看使用while结构来遍历集合中的每个元素

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    // 使用for循环这种更简明的方法来遍历集合中的每一个元素
    // 增强了代码的安全性，不会出现诸如越界访问或漏掉某些元素之类的问题。

    for item in a.iter() {
        println!("the value is: {}", item);
    }

    // range 被用来生成从一个数字开始到另一个数字结束之前的所有数字序列
    // rev方法可以翻转序列

    for num in (1..4).rev() {
        println!("{}!", num);
    }

    println!("LIFTOFF!!!");
}
