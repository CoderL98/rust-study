use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("开始猜数游戏!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..100);

    loop {
        println!("请输入你要猜的数:");

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess
            // 去除空字符 换行 输入后回车默认带一个\n
            .trim()
            // 转换为整数
            .parse()
        // .expect("请输入一个数字！！！");
        {
            Ok(num) => num,
            Err(_) => {
                println!("您输入的内容为: {},请输入一个数字", guess.trim());
                continue;
            },
        };

        println!("你猜测的数是: {}", guess);

        // println!("生成的随机数字是: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("To small")
            }
            Ordering::Greater => {
                println!("To big");
            }
            Ordering::Equal => {
                println!("You win!");
                println!("Game over!");
                break;
            }
        }
    }
}
