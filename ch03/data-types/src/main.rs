use rand::Rng;

fn main() {
    {
        // 整型 (i|u)(8|16|32|64|128|size) 数字是bit的数目  u表示无符号 默认是i32
        // let num_u8: u8 = -1; // error
        let num_u8: u8 = u8::MAX;
        let num_i8: i8 = i8::MAX;
        println!("The value of num_u8 is: {num_u8}");
        println!("The value of num_i8 is: {num_i8}");

        let num_u16: u16 = u16::MAX;
        let num_i16: i16 = i16::MAX;
        println!("The value of num_u16 is: {num_u16}");
        println!("The value of num_i16 is: {num_i16}");

        let num_u32: u32 = u32::MAX;
        let num_i32: i32 = i32::MAX;
        println!("The value of num_u32 is: {num_u32}");
        println!("The value of num_i32 is: {num_i32}");

        let num_u64: u64 = u64::MAX;
        let num_i64: i64 = i64::MAX;
        println!("The value of num_u64 is: {num_u64}");
        println!("The value of num_i64 is: {num_i64}");

        let num_u128: u128 = u128::MAX;
        let num_i128: i128 = i128::MAX;
        println!("The value of num_u128 is: {num_u128}");
        println!("The value of num_i128 is: {num_i128}");

        let num_usize: usize = usize::MAX; // size是当前允许系统架构的位数，32为系统是32，64为系统是64
        let num_isize: isize = isize::MAX;
        println!("The value of num_usize is: {num_usize}");
        println!("The value of num_isize is: {num_isize}");

        {
            // 整形字面值
            let num_dec: i32 = 100_000; // 普通的10进制 可以使用 _ 分割数字，使其具有较好的可读性
            let num_hex: i32 = 0xff_ff; // 16进制
            let num_oct: i32 = 0o77_77; // 8进制
            let num_bin: i32 = 0b1111_1111; // 2进制
                                            // let num_byte: i32 = b'A'; // 单字节，仅限于u8
            let num_byte: u8 = b'A'; // 单字节，仅限于u8
            println!("The value of num_dec is: {num_dec}");
            println!("The value of num_hex is: {num_hex}");
            println!("The value of num_oct is: {num_oct}");
            println!("The value of num_bin is: {num_bin}");
            println!("The value of num_byte is: {num_byte}");
        }

        // 整型溢出，rust在debug模式下如果出现溢出会报错，在release模式下会补码
        let num_i32_input: i32 = rand::thread_rng().gen_range(1..=100);

        // let num_i32_wrapping = num_i32_input + i32::MAX; // debug 运行报错 release运行正常
        // 如果溢出是可接受的，可以使用rust提供的方法 wrapping_xx 运行溢出，溢出后按照补码逻辑进行
        let num_i32_wrapping = num_i32_input.wrapping_add(i32::MAX);
        println!("The value of num_i32_max is: {num_i32_wrapping}");

        // checked_xxx 进行计算，如果溢出，返回为none
        if num_i32_input.checked_add(i32::MAX).is_none() {
            // 可以检测是否会溢出
            println!("Will integer overflow");
        }

        // overflowing_xxx 返回计算值(会溢出)，以及是否发生溢出
        let (overflowing_add_res, overflowing_add_res_is_overflow) =
            i32::MAX.overflowing_add(i32::MAX);
        println!("The value of overflowing_add_res is: {overflowing_add_res}, overflowing_add_res_is_overflow is: {overflowing_add_res_is_overflow}");

        // saturating_xxx 返回安全范围内的最大最小值
        let saturating_add_res = i32::MAX.saturating_add(i32::MAX);
        println!("The value of saturating_add_res is: {saturating_add_res}");
    }

    {
        // 浮点型，rust只有两种浮点型 f64 f32 默认为f64
        let x = 2.0;
        let y: f32 = 0.1 + 0.2;
        println!("The value of x is: {x}");
        println!("The value of y is: {y}");
    }

    {
        // 布尔值,和其他一般语言一样，是 ture 和 false
        let yes = true;
        let no: bool = false;
        println!("The value of yes is: {yes}");
        println!("The value of no is: {no}");
    }

    {
        // 字符类型 rust中字符的大小为4个字节，所以可以输入一个emoji
        let c = 'z';
        let z: char = 'Z';
        let emoji = '😀';
        println!("The value of c is: {c}");
        println!("The value of z is: {z}");
        println!("The value of emoji is: {emoji}");
    }

    {
        // 复合类型

        {
            // 元组类型，可以将多个不同类型的值组合到一起
            let tup: (i32, f64, u8) = (500, 6.4, 1);

            println!("The value of tup.0 is: {}", tup.0);
            println!("The value of tup.1 is: {}", tup.1);
            println!("The value of tup.2 is: {}", tup.2);

            let (x, y, z) = tup;
            println!("The value of x is: {x}");
            println!("The value of y is: {y}");
            println!("The value of z is: {z}");
        }

        {
            // 数组类型，类型必须相同
            let a = [1, 2, 3, 4, 5]; // 直接初始化每个项

            let second = a[1];
            println!("The value of second is: {}", second);

            let b = [3; 5]; // 初始化长度为5的数组，使用3填充
            let second = b[1];
            println!("The value of second is: {}", second);

            // 超过下标访问会报错
            // let random_index: usize = rand::thread_rng().gen_range(10..=100);
            // let element = b[random_index];
        }
    }
}
