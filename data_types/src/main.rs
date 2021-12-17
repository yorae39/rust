use std::io;

fn main() {
   
    //use_float_point();

    //use_char();

    //mathematical_operations();

    //use_boolean();

    //use_tuple();

    //use_tuple_for_index();

    //use_arrays();

    //declared_type_array();

    //accessing_element_in_array();
    example_protect_invalid_access_memory();

}

/*fn use_float_point() {
        
    //PONTO FLUTUANTE
    let x = 2.1; // f64 -> NÃO IMPRIME ZERO

    let y: f32 = 3.1; // f32 -> NÃO IMPRIME ZERO

    println!("64 bits: {}, 32 bits: {}", x, y);
}*/

/*fn use_char() {
    //CHAR
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    
    println!("c = {}, z = {}, cat = {}",  c, z, heart_eyed_cat);
}*/

/*fn mathematical_operations() {
    
    //FUNÇÕES MATEMÁTICAS

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("addition: {}, subtraction: {}, multiplication: {}, 
              division (quotient): {},  division (floored): {}, remainder: {}",
            sum, difference, product, quotient, floored, remainder)
}*/

/*fn use_boolean() {
    let t = true;

    let f: bool = false; // com anotação de tipo explícito

    println!("Booleano 1: {}, Booleano 2: {}", t, f)
}*/

/*fn use_tuple() {
    //TIPOS  - TUPLAS PODE CONTER VÁRIOS TIPOS E É DE TAMANHO FIXO
    let tup: (i32, f64, u8) = (500, 64.1, 1);

    //DESESTRUTURAÇÃO
    let (x, y, z) = tup;

    println!("Value of x in tuple: {}, Value of y in tuple: {}, Value of z in tuple: {}", x, y, z);
}*/

/*fn use_tuple_for_index() {

    let x: (i32, f64, u8) = (500, 6.1, 1);
    
    let five_hundred = x.0;
    let six_point_one = x.1;
    let one = x.2;

    println!("Value of x for index 0: {}, Value of x for index 1: {}, Value of x for index 2: {}",
    five_hundred, six_point_one, one);
}*/

/*fn use_arrays() {    
    //TAMBÉM SÃO DE TAMANHO FIXO
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("Months:");
    for month in  months {
        println!("  {}", month)
    }
}*/

/*fn declared_type_array() {
    //DECLARANDO O TIPO E O TAMANHO DE UMA MATRIZ - ARRAY
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //ATALHO PARA CRIAÇÃO DE UM MESMO TIPO E E MESMO VALOR
    let b = [3; 5];

    println!("Value of a: {:?}", a); //FORMATADOR - :?

    println!("Value of b: {:?}", b);

}*/


/*fn accessing_element_in_array() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("First element in a: {}", first);
    println!("Second element in a: {}", second);
}*/

//RUST PROTEGENDO ACESSO A MEMÓRIA INVÁLIDA

/*
NO CASO DE FORNRCER UM INDEX INVÁLIDO:
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5', src\main.rs:154:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\data_types.exe` (exit code: 101)
*/
fn example_protect_invalid_access_memory() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}