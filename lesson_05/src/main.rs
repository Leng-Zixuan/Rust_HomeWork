macro_rules! my_macro {
    ($x:expr) => {
        println!("The value of x is: {}", $x);
    };
}

fn main() {
    println!("Hello, world!");
}
/*
这个声明宏的名称是my_macro，它接受一个表达式$x作为参数，并打印出$x的值。在这个示例中，我们使用了macro_rules!关键字来定义宏，这是Rust中定义声明宏的方式。

宏定义的语法类似于模式匹配，其中$x:expr表示一个表达式模式，=>表示模式匹配成功后要执行的代码块。在这个示例中，我们只有一个模式，即($x:expr)，它表示宏接受一个表达式作为参数。

声明宏的编译过程是在编译时进行的。当您编译包含声明宏的代码时，Rust编译器会将宏展开为相应的代码，并将其插入到您的程序中。这意味着声明宏的代码本身不会出现在最终的可执行文件中，而是在编译时被替换为相应的代码。

在使用声明宏时，您可以像调用函数一样调用宏，并将参数传递给宏。在编译时，Rust编译器会将宏展开为相应的代码，并将其插入到您的程序中。这使得声明宏成为了一种非常强大的代码生成工具，可以帮助您减少重复的代码和提高代码的可读性。
*/
