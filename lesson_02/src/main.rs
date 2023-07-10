fn main() {
    /*
    NO.1
    所有权规则：每个值都有一个所有者，值只能有一个所有者，当所有者离开作用域时，值将被丢弃。
    所有权特性：Rust 通过所有权系统来管理内存，避免了内存泄漏和数据竞争等问题。
    */
    // s1 是 String 类型的值，它拥有所有权，当 s1 离开作用域时，它的所有权将被丢弃
    let s1 = String::from("hello");
    println!("{}", s1);

    /*
    NO.2
    不可变引用规则：可以有多个不可变引用，但不能有可变引用，不可变引用不能修改值。
    不可变引用特性：不可变引用可以与其他不可变引用共享数据，但不能与可变引用共享数据。
     */
    let s2 = String::from("hello");
    // 传递 s2 的引用给函数
    let len: usize = calculate_length(&s2);

    println!("The length of '{}' is {}.", s2, len); // s2 仍然有效

    fn calculate_length(x: &String) -> usize {
        // x 是一个指向 String 类型值的引用
        x.len()
    }

    /*
    NO.3
    可变引用规则：只能有一个可变引用，不能有不可变引用，可变引用不能与其他引用共享数据。
    可变引用特性：可变引用可以修改值，但不能与其他引用共享数据。
    */
    let mut s3 = String::from("hello");

    change(&mut s3); // 传递 s3 的可变引用给函数

    println!("{}", s3); // s3 的值已经被修改

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
}
