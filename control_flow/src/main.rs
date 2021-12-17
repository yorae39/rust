fn main() {
  
    //use_if_else();
    //println!("{}", use_match(27));
    //inner_loops();

    //return_value_in_loops(8);

    //loop_while();
    //index_loop();

    //loop_for();

    //loop_for_mut();

    //loop_for_not_mut();

    rev_loop();

}


/*fn use_if_else() {
    let number = 6;

    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 4 == 0 {
        println!("number is divisible by 4");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}*/

/*fn use_match(value: u32) -> &'static str {
    let msg;
    match value {
        v if v % 2 == 0 => msg = "number is divisible by 2",
        v if v % 3 == 0 => msg = "number is divisible by 3",
        v if v % 4 == 0 => msg = "number is divisible by 4", //NUNCA SERÁ EXECUTADO
        _ => msg = "other"
    }
    return msg;
}*/

/*fn inner_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}*/

/*fn return_value_in_loops(until_value: u32) {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == until_value {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}*/


/*fn loop_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}*/

/*fn index_loop() {
    let a = [10, 20, 30, 40, 50, 60];
    let mut index = 0;

    while index < a.len() {
        println!("In index = {} the value is: {}", index, a[index]);

        index += 1;
    }
}*/


/*fn loop_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}",  element);
    }
}*/

/* fn loop_for_mut() {

    let mut a = [10, 20, 30, 40, 50];

    for (i, elem) in a.iter_mut().enumerate() {
        // *elem += i as u8; //CASO QUEIRA ATUALIZAR OS VALORES DA MATRIZ
        println!("Index = {} equal value = {}", i, elem);
    }
}*/


/*fn loop_for_not_mut() {
    let a = [10, 20, 30, 40, 50];

    for (i, elem) in a.iter().enumerate() {
        println!("Index = {} equal value = {}", i, elem);
    }
}*/

fn rev_loop() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}