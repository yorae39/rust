fn main() {
   
    //mut - FAZ A VARIÁVEL SE TORNAR MUTÁVEL
    let mut x = 5;
    println!("The value of x is {}", x);

    //shadowing
    x = 6;
    println!("The value of x is {}", x);

    {
        //shadowing
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }


    //const - CONSTATNTES SÃO SEMPRE IMUTÁVEIS
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Constant: {}", THREE_HOURS_IN_SECONDS);

    //SOMBREAMENTO COM TIPOS DIFERENTES
    let spaces =  " ";
    let spaces = spaces.len();

    println!("{}", spaces)

    //spaces = spaces.len(); //GERA UM ERRO DE COMPILAÇÃO
}
