// 第一种
enum ThreeTypes {
    TypeA(i32),
    TypeB(String),
    TypeC(bool),
}

impl ThreeTypes {
    fn do_something_a(&self) {
        if let ThreeTypes::TypeA(num) = self {
            println!("TypeA: {}", num);
        }
    }

    fn do_something_b(&self) {
        if let ThreeTypes::TypeB(string) = self {
            println!("TypeB: {}", string);
        }
    }

    fn do_something_c(&self) {
        if let ThreeTypes::TypeC(boolean) = self {
            println!("TypeC: {}", boolean);
        }
    }
}

//第二种
trait DoSomething {
    fn do_something(&self);
}

struct TypeA(i32);

impl DoSomething for TypeA {
    fn do_something(&self) {
        println!("TypeA: {}", self.0);
    }
}

struct TypeB(String);

impl DoSomething for TypeB {
    fn do_something(&self) {
        println!("TypeB: {}", self.0);
    }
}

struct TypeC(bool);

impl DoSomething for TypeC {
    fn do_something(&self) {
        println!("TypeC: {}", self.0);
    }
}

fn main() {
    /*
       区别：
           第一种：枚举类型，每个枚举值都有自己的类型，所以需要分别实现每个枚举值的方法
           第二种：trait，所有类型都实现了trait，所以只需要实现trait的方法即可
           第一个问题使用枚举来包装三个不同的类型，而第二个问题使用Trait Object来定义三个不同的类型。
    */
    // 第一种
    let vec = vec![
        ThreeTypes::TypeA(42),
        ThreeTypes::TypeB(String::from("Hello, world!")),
        ThreeTypes::TypeC(true),
    ];

    for item in vec {
        item.do_something_a();
        item.do_something_b();
        item.do_something_c();
    }

    // 第二种
    let vec: Vec<Box<dyn DoSomething>> = vec![
        Box::new(TypeA(42)),
        Box::new(TypeB(String::from("Hello, world!"))),
        Box::new(TypeC(true)),
    ];

    for item in vec {
        item.do_something();
    }
}
