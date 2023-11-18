fn main() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // 默认不可变，需要手动加上mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // 常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // 隐藏，可以隐藏之前的不可变变量，用于临时计算
    let x = 5;
    let x = x + 1;
    {
        // 块级作用域
        let x = x * 2;
        println!("The Value of x in the inner scope is: {x}"); // 12
    }
    println!("The value of x is: {x}"); // 6
}
