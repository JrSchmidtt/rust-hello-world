const PI:f32 = 3.1415;           // constante de tipo f32 escopo global
static mut GLOBAL: i32 = 10;     // variável de tipo i32 escopo global pode ser mutavel porem é thread safe

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

fn main() {
    escopo();
    println!("soma: {}", soma(10, 20)); // ; discarta o resultado e uma função
}