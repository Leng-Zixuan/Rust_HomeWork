fn q_one() {
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
}

fn q_two() {
    // 第二种
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

    let vec: Vec<Box<dyn DoSomething>> = vec![
        Box::new(TypeA(42)),
        Box::new(TypeB(String::from("Hello, world!"))),
        Box::new(TypeC(true)),
    ];

    for item in vec {
        item.do_something();
    }
}

fn q_custom_add() {
    trait Add<RHS, Output> {
        fn custom_add(self, rhs: RHS) -> Output;
    }

    impl Add<i32, i32> for i32 {
        fn custom_add(self, rhs: i32) -> i32 {
            self + rhs
        }
    }

    impl Add<f64, f64> for f64 {
        fn custom_add(self, rhs: f64) -> f64 {
            self + rhs
        }
    }

    let a = 1;
    let b = 2;
    let c = 1.5;
    let d = 2.5;

    let vec: Vec<Box<dyn Add<_, _>>> = vec![Box::new(a), Box::new(b), Box::new(c), Box::new(d)];

    for item in vec {
        println!("{}", item.custom_add(3));
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
    q_one();

    // 第二种
    q_two();

    q_custom_add();
}
