fn main() {
    {
        // 通过fn申明函数
        fn another_function() {
            println!("Another function.");
        }
        another_function();
    }

    {
        // 参数定义 参数必须声明类型
        fn another_function(x: i32) {
            println!("the value of x is: {x}");
        }
        another_function(5);

        // let num: u32 = 10;
        // another_function(num); // 会报错

        // rust中赋值语句 不会返回值
        // 语句 不会返回值
        // 表达式 会返回值
        // let x = (let y = 1); // 会报错

        // 可以通过如下形式实现类似的能力
        let x = {
            let y = 1;
            y // 不加上分号则是一个表达式，加上则是一个语句
        };
        println!("the value of x is: {x}");
    }

    {
        // 返回值 -> 指定返回值类型
        fn another_function(x: i32) -> i32 {
            return x + 1;
        }
        let x = another_function(1);
        println!("the value of x is: {x}"); // 2

        // 也可以直接是一个表达式，最后一行没有分号，块代码就是表达式
        fn another_function2(x: i32) -> i32 {
            x + 1
        }
        let x = another_function2(1);
        println!("the value of x is: {x}"); // 2
    }
}
