fn main() {
    let mut str = "1";
    str = "2";

    let x = 1;
    let x = 2;
    println!("Hello, world!{}{}", str, x);

    // 只有字符串可以打印 非字符串不可以
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // 不加mut才可以变化类型 否则不行
    let spaces = "     ";
    let spaces = spaces.len();

    println!("{}", spaces);

    // 大部分场景可以猜测类型 但是类型可能较多的时候 必须手动指定类型 否则编译报错
    let guess: i32 = "42".parse().expect("Not a number");

    println!("{}", guess);

    /*
    * u32 无符号整数类型 占据32位空间 范围从0~(2的n次方 - 1) 包括两头
    * i 有符号整数类型 占据32位空间 范围从 -(2的n次方 - 1)~(2的n-1次方 - 1)
    * iu8 iu16 iu32 iu64 iu128 iusize
    * isize和usize的位数有程序运行的计算机的架构所决定 64位计算机 则为64 32位计算机 则为32位
    * isize和usize的使用场景是对某种集合进行索引操作 一般用不到！！！
    *
    * 整数字面值
    * 十进制 采用_分隔 10_000
    * 十六进制 采用0x开头 0xff
    * 八进制 采用0o开头 0o77
    * 二进制 采用0b开头 0b1111_0000
    *
    */
}
