// fn main() {
//     println!("Hello, world!");
//     // let a = 10;
//     // println!("a = {:?}", a);
//
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
    //
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    //
    // println!("The value of x is: {}" ,x);
    //
    // let mut spaces = "    ";
    // spaces = spaces.len();

    // let guess = "42".parse().expect("Not a Number!");

    // let a = [1,2,3,4,5];
    // let index = 10;
    //
    // let element = a[index];
    //
    // println!("The value of element is: {}", element);

// }
use std::io;
fn main() {
//     let x = plus_one(5);
//
//     println!("The value of x is : {}", x);
//     println!("Exercise 1 : 정수 1부터 100까지 더하여 화면에 출력하는 프로그램을 작성하세요");
//     let mut result : i64 = 0;
//     for number in 1..101{
//         result = result+ number;
//     }
//     println!("Exercise 1 Answer : {}", result);
//     println!("Exercise 2 : 터미널에서 문자열을 입력받아서 그 문자열을 역순으로 출력하세요");
//
//     let mut string  = String::new();
//
//     io::stdin().read_line(&mut string)
//         .expect("Failed to read line");
//     let answer = string.chars().rev().collect::<String>();
//     println!("Exercise 2 Answer : {}", answer);
//
//
//     println!("Exercise 3 : 임의의 숫자를 입력 받고 그 숫자를 20자리의 자릿수로 출력");
//
//
//     loop {
//         let mut val = String::new() ;
//         io::stdin().read_line(&mut val)
//             .expect("Failed to read line");
//         let val: i64 =  match val.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         println!("Exercise 3 Answer : {}", change(val));
//         break;
//     }


    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
//
fn change(x : i64) -> String{
    let len = x.to_string().len() ;

    let commaCount = if len % 3 == 0 {
         len / 3 -1
    }
    else {
            len / 3
    };
    let zeroCount = 20-len;

    let mut vector =  vec![];

    let mut count = 0;

    for char in x.to_string().chars().rev() {

        if count == 3 {
            vector.insert(0, ',');
            vector.insert(0, char);
            count = 1;
        }
        else {
            vector.insert(0, char);
            count = count + 1;
        }
    }
    let arr = vec!['0'; zeroCount];
    for zero in arr {
        vector.insert(0, zero);
    }
    println!("{:?}",vector);
    vector.iter().collect()
}
