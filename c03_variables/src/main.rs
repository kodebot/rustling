fn main() {
    // not mutable
    let x = 5;
    println!("the value of x: {}", x);

    // shadowing and mutable
    let mut x = 7;
    println!("the value of x before mutation is: {}", x);
    x = 8;
    println!("the value of x now is: {}", x);

    // must have type annotation
    const PI: f64 = 3.14;
    println!("the const value of PI: {}", PI);

    // readable
    const MILLION: u32 = 1_000_000;
    println!("the const value of MILLION: {}", MILLION);

    // function
    let a = 10;
    let b = 20;
    println!("sum of {}, {} is {}", a, b, add(a, b));

    // control flow

    // if..else
    let c = 100;
    if c < 100 {
        println!("c is less than 100");
    } else {
        println!("c is not less than 100");
    }

    // loop - unconditional
    let mut counter = 10;
    loop {
        if counter == 0 {
            break;
        }

        println!("counter is {}", counter);
        counter = counter - 1;
    }

    // while
    let mut counter1 = 10;
    while counter1 > 0 {
        println!("counter1 is {}", counter1);
        counter1 -= 1;
    }

    // for
    let list = [1, 2, 3, 4, 5];
    for item in list.iter() {
        println!("current item is: {}", item);
    }

    for n in 1..10 {
        println!("current number in for range is: {}", n);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
