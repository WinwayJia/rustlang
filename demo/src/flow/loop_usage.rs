pub fn loop_demo() {
    let mut i = 0;
    loop {
        println!("again");
        i = i + 1;
        if i > 5 {
            break;
        }
    }
}

pub fn loop_return() {
    let mut counter  = 0;
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter*2;
        }
    };
    
    println!("result: {}", result)
}

pub fn while_demo() {
    let mut i = 3;
    while i>0 {
        println!("i = {}", i);
        i -= 1;
    }

}

pub fn for_demo() {
    let arr = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i  < 5 {
        println!("arr[{}] = {}", i, arr[i]);
        i += 1;
    }

    for e in arr.iter() {
        println!("{}", e);
    }

    for i in 1..3 {
        println!("rev = {}", i);
    }

    for i in (1..3).rev() {
        println!("rev = {}", i);
    }
}

pub fn fab(n :i32) {
    let mut first = 1;
    let mut second  = 1;

    if n <= 2 {
        println!("fab({})=1", n);
        return;
    } 

    let mut result = 0;
    let mut i = 3;
    while i <= n {
        result = first + second;
        first = second;
        second = result;
        i += 1;
    }
    println!("fab({})={}", n, result);
}