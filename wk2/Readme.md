模仿老师的思路，自己对所有权、不可变引用、可变引用这三者的规则或特性做一个
集中的总结，写一个笔记列表。

所有权:
    fn main() {
        let original_string = String::from("Hello, Rust!");

        // Move the ownership of `original_string` to `new_string`
        let new_string = original_string;

        // Trying to use `original_string` after the move will result in a compilation error
        println!("Original string: {}", original_string);
    }
以上例子是rust对所有权的监管。所有权已经被转移到新的名言，久的名言不能用了。看起来好像没问题的代码也不能编译。有两种解决方法。第一就是clone。第二就是去掉String,用str。

如是要更改字段，原字段就要用String type。 不可以用str type. 原来的名言必须是 mut ，所谓的可更改的函数。

let mut original_string = String::from("Hello, Rust!");
original_string.push_str("added");