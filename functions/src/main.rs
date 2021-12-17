fn main() {
  use_expression_in_function();
  use_return_value_function(47);
}

//NÃO TEVE NADA MUITO INTERESSANTE NA PARTE DE FUNÇÕES, ENTÃO SEPAREI DOIS EXEMPLOS

fn use_expression_in_function() {
   
    let y = {

        let x = 3;
        //As expressões não incluem ponto e vírgula final.
        x * 2
    };

    println!("Value of y is: {}", y)
}

//FUNÇÕES COM RETORNO

fn function_return_value(x:i32) -> i32 {
    x + 1
}

fn use_return_value_function(value:i32) {
    let result = function_return_value(value);
    println!("Value of return function is: {}", result)
}