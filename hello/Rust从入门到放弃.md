
# Hello World

## 安装


## hello world


## cargo

```ruby
$ cargo --version

$ cargo new hello_cargo 
$ cd hello_cargo


# 也可以简单地使用cargo run命令来依次完成编译和运行任务
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!


# 也可以简单地使用cargo run命令来依次完成编译和运行任务。 如果代码没有变化，这一步可能省略掉build编译流程
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!


# cargo check的命令，你可以使用这个命令来快速检查当前的代码是否可以通过编译，而不需要花费额外的时间去真正生成可执行程序
$ cargo check
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```
> cargo check的命令，你可以使用这个命令来快速检查当前的代码是否可以通过编译，而不需要花费额外的时间去真正生成可执行程序


- Cargo.toml
  - Cargo使用TOML（Tom's Obvious, Minimal Language）作为标准的配置格式
    - [package]是一个区域标签，它表明接下来的语句会被用于配置当前的程序包
    - [dependencies]同样是一个区域标签，它表明随后的区域会被用来声明项目的依赖。


当准备好发布自己的项目时，你可以使用命令cargo build --release在优化模式下构建并生成可执行程序。它生成的可执行文件会被放置在target/release目录下，而不是之前的target/debug目录下。


```ruby

```

let、match、方法、关联函数，以及外部包的使用

expect

Result

cmp

Ordering


# 基础知识

## 数据类型

rust是一门静态类型语言。——编译程序的过程中需要知道所有变量的具体类型。

**显式的类型标注**


```rust
let guess:u32 = "42".parse().expect("Not a number!");  // :u32显式的添加的类型标注
let guess = "42".parse().expect("Not a number!"); // 会报错 can't infer type for `_`
```


### 标量类型 scalar

标量类型是单个值类型的统称。

内建了4种基础的标量类型： 整书、浮点数、布尔值、字符。


#### 整数
u32 无符号
i32 有符号

其他（8/16/64），有符号数是通过二进制补码的形式存储。
- 还有isize、usize 两种特殊的整书类型，它们的长度取决于程序运行的目标平台。
  - 在64位架构上，就是64位
  - 在32位架构上，就是32位
- 除了Byte，其余所有的字面量都可以使用类型后缀，比如57u8，代表一个使用了u8类型的整数57。
- 可以使用_作为分隔符以方便读数，比如1_000。

Rust对于整数字面量的**默认推导类型i32**通常就是一个很好的选择：它在大部分情形下都是运算速度最快的那一个，即便是在64位系统上也是如此。

较为特殊的两个整数类型usize和isize则主要用作某些集合的索引。

#### 浮点数
f32/f64

由于在现代CPU中f64与f32的运行效率相差无几，却拥有更高的精度，所以在Rust中，默认会将浮点数字面量的类型推导为f64。

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```


#### 布尔值

Rust的布尔类型只拥有两个可能的值：true和false，它会占据单个字节的空间大小。

```rust
 fn main() { 
    let t = true; 
 
    let f: bool = false; // 附带了显式类型标注的语句 
} 
```


#### 字符

char类型被用于描述语言中最基础的单个字符。

使用单引号指定，字符串是双引号。


char类型占4字节，是一个Unicode标量值，这也意味着它可以表示比ASCII多得多的字符内容。 可以表示emoji。

### 复合类型 compound

char类型占4字节，是一个Unicode标量值，这也意味着它可以表示比ASCII多得多的字符内容


### 复合类型
Rust提供了两种内置的基础复合类型：元组（tuple）和数组（array）。

#### 元组

- 它可以将其他不同类型的多个值组合进一个复合类型中。

- 元组还拥有一个固定的长度：你无法在声明结束后增加或减少其中的元素数量。

- 为了创建元组，我们需要把一系列的值使用逗号分隔后放置到一对圆括号中。

>元组每个位置的值都有一个类型，这些类型不需要是相同的。

```rust
fn main() {
  let tup = (500, 6.4, 1);
}

```

- 获取元组单个值： 
  - 解构
  ```rust
  fn main() { 
      let tup = (500, 6.4, 1); 
  
      let (x, y, z) = tup;   // 这个操作被称为 解构
  
      println!("The value of y is: {}", y); 
  } 
  ```
  - 通过索引+点号
  ```rust
  fn main() {
    let x:(i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
  }  

  ```



#### 数组

- 数组要求每个元素的类型必须相同。

- 数组拥有固定的长度，一旦声明就不能改变。

**创建**：

```rust
fn main() {
  let a = [1,2,3,4,5];
}
```

>通常而言，当你想在栈上而不是堆上为数据分配空间时，或者想要确保总有固定数量的元素时，数组是一个非常有用的工具


也有 **动态数组**： Rust标准库也提供了一个更加灵活的动态数组（vector）类型。
- 动态数组是一个类似于数组的集合结构，但它允许用户自由地调整数组长度。
- 假如你还不确定什么时候应该使用数组，什么时候应该使用动态数组，那就先使用动态数组好了。


如果你知道数组的长度，那就选择数组，比如表示12个月的名字等。


为了**写出数组的类型**，你需要使用一对方括号，并在方括号中填写数组内所有元素的类型、一个分号及数组内元素的数量
```rust
let a:[i32；5] = [1,2,3,4,5];
```


一种定义重复元素的数组的简洁方法：
```rust
let a = [3;5];
等价于
let a = [3,3,3,3,3,3];
```


**访问数组的元素**

数组由一整块分配在栈上的内存组成，你可以通过索引来访问一个数组中的所有元素.

```rust
fn main() { 
    let a = [1, 2, 3, 4, 5]; 
 
    let first = a[0]; 
    let second = a[1]; 
} 
```

**非法的数组元素访问**


越界： 程序会顺利通过编译，却会在运行时因为错误而崩溃退出。

编译并没有产生任何错误提示，但是程序却因为一个运行时错误而不正确地终止了运行。

```
有许多底层语言没有提供类似的检查，一旦尝试使用非法索引，你就会访问到某块无效的内存。
在这种情况下，逻辑上的错误常常会蔓延至程序的其他部分，进而产生无法预料的结果。
```
>上面说的真的嘛？ Java这种语言不都有非法越界的报错吗？ 为啥这里作则说很多语言都没有？ 底层语言？


--- 
## 函数

rust使用snake case作为规范函数和变量名称的风格。 小写字母，下划线分割。

```rust
 fn main() { 
    println!("Hello, world!"); 
 
    another_function(); 
} 
 
fn another_function() { 
    println!("Another function."); 
} 
```

我们在这个例子中将another_function函数定义在了main函数之后，但把它放到main函数之前其实也没有什么影响。

>Rust不关心你在何处定义函数，只要这些定义**对于使用区域是可见**的即可。


### 函数参数

在英语技术文档中，参数变量和传入的具体参数值有自己分别对应的名称parameter和argument，
但我们通常会混用两者并将它们统一地称为参数而不加以区别。


在函数签名中，你必须显式地声明每个参数的类型。
--由于类型被显式地注明了，因此编译器不需要通过其他部分的代码进行推导就能明确地知道你的意图。



### 函数体中的语句和表达式

函数体由若干条语句组成，并可以以一个表达式作为结尾。

**Rust是一门基于表达式的语言**, 所以它将语句（statement）与表达式（expression）区别为两个不同的概念，这与其他某些语言不同。

- 语句指那些执行操作但不返回值的指令。
- 而表达式则是指会进行计算并产生一个值作为结果的指令。


**要记住，语句不会返回值。**


因此，在Rust中，你不能将一条let语句赋值给另一个变量，如下所示的代码会产生编译时错误：

```rust
fn main() {
  let x = (let y = 6);
}
```


**表达式会计算出某个值来作为结果**

>你在Rust中编写的大部分代码都会是表达式。

以简单的数学运算5 + 6为例，这就是一个表达式，并且会计算出值11。

- 表达式本身也可以作为语句的一部分。
  - 在示例3-1中，语句`let y = 6;`中的字面量6就是一个表达式，它返回6作为自己的计算结果。
  - 调用函数是表达式
  - 调用宏是表达式
  - 我们用来创建新作用域的花括号（{}）同样也是表达式.

```rust
fn main() { 
    let x = 5; 
 
 ❶ let y = {❷ 
         let x = 3; 
     ❸  x + 1 
    }; 
 
    println!("The value of y is: {}", y); 
} 
```

1. 表达式❷是一个代码块。在这个例子中，它会计算出4作为结果。
2. 而这个结果会作为let语句❶的一部分被绑定到变量y上。
3. 注意结尾处❸的表达式x + 1**没有添加分号**，这与我们之前见过的大部分代码不同。
4. 假如我们在表达式的末尾**加上了分号**，这一段代码就**变为了语句而不会返回任何值**。


**函数的返回值**


- 函数可以向调用它的代码返回值。

- 虽然你不用为这个返回值命名，但需要在箭头符号（->）的后面声明它的类型。

- 在Rust中，函数的返回值等同于函数体最后一个表达式的值。

```rust
fn five() -> i32 {
  5
}

fn main() {
  let x = five();
  println!("The value of y is: {}", x); 
}
```

## 控制流


### if表达式

```rust
fn main() { 
    let number = 3; 
 
    if number < 5 { 
        println!("condition was true"); 
    } else { 
        println!("condition was false"); 
    } 
} 
```

- 代码中的条件表达式必须产生一个bool类型的值，否则就会触发编译错误。 字符串"1"、数字1/0都不行。
- 太多分支其他语言有switch，Rust中用强大的分支结构语法match来应对这种情况


**在let语句中使用if**

由于if是一个表达式，所以我们可以在let语句的右侧使用它来生成一个值，如示例：

```rust
fn main() {
  let condition = true;
  let number = if condition {
    5
  } else {
    6
  };
  println!("The value of number is: {}", number);
}
```

在上面的例子中，整个if表达式的值取决于究竟哪一个代码块得到了执行。这也意味着，

所有if分支可能返回的值都必须是一种类型的；


#### 使用循环重复执行代码


- loop
- while
- for



# 认识所有权

**所有权可以说是Rust中最为独特的一个功能了。**

- 正是所有权概念和相关工具的引入，Rust才能够在没有垃圾回收机制的前提下保障内存安全。

相关内容： **借用、切片，以及Rust在内存中布局数据的方式。**
