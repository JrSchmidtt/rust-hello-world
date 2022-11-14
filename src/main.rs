const PI:f32 = 3.1415;           // constante de tipo f32 escopo global
static mut GLOBAL: i32 = 10;     // variável de tipo i32 escopo global pode ser mutavel porem é thread safe

fn main() {
    escopo();
    println!("soma: {}", soma(10, 20)); // ; discarta o resultado e uma função
    maiorDeIdade(18);
    repeticoes();
    forExemplo();
    ownership();
}

fn escopo(){
    let inteiro:i32 = 128;       // inteiro de 32 bytes
    let decimal:f32 = 0.5;       // float de 32 bytes
    let booleano:bool = true;    // booleano
    let letra:char = 'a';        // caracter

    unsafe {
        println!("varial global: {} tamanho {} bytes", GLOBAL, std::mem::size_of_val(&GLOBAL));
    }

    println!("PI: {} tamanho {} bytes", PI, std::mem::size_of_val(&PI));
    println!("inteiro: {} tamanho {} bytes", inteiro, std::mem::size_of_val(&inteiro));
    println!("decimal: {} tamanho {} bytes", decimal, std::mem::size_of_val(&decimal));
    println!("booleano: {} tamanho {} bytes", booleano, std::mem::size_of_val(&booleano));
    println!("letra: {} tamanho {} bytes", letra, std::mem::size_of_val(&letra));

    println!("Hello, world!");      // println! é uma macro não uma função
}

fn soma(y: i32, x: i32) -> i32 {
    return y + x;
}

fn maiorDeIdade(idade: i32) {
    if idade >= 18 {
        println!("maior de idade");
    } else {
        println!("menor de idade");
    }
}

fn repeticoes(){
    let mutiplicador:u8 = 5;
    let mut contador:u8 = 0;
    while contador < 10 {
        contador += 1;
        println!("{} x {} = {}", mutiplicador, contador, mutiplicador * contador);
    }
    contador = 0;
    loop {
        contador += 1;
        println!("{}", contador);
        if contador == 10 {
            break;
        }
    }
}

fn forExemplo(){
    for x in 1..=10 {
        println!("{}", x);
    }
}

fn ownership(){
    let mut s1 = String::from("hello");     // aloca na heap uma var mutavel
    empresta(&mut s1);                      // passa como paramentro uma referencia de memoria mutavel
    println!("{}, world!", s1);       
}

fn empresta(s1: &mut String) {          // recebe uma refencia mutavel
    s1.push_str(" Mudou");
    println!("{}", s1);
}