
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