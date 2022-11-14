fn main() {
    let inteiro:i32 = 128;          // inteiro de 32 bytes
    let decimal:f32 = 0.5;          // float de 32 bytes
    let booleano:bool = true;       // booleano
    let letra:char = 'a';           // caracter

    println!("inteiro: {} tamanho {} bytes", inteiro, std::mem::size_of_val(&inteiro));
    println!("decimal: {} tamanho {} bytes", decimal, std::mem::size_of_val(&decimal));
    println!("booleano: {} tamanho {} bytes", booleano, std::mem::size_of_val(&booleano));
    println!("letra: {} tamanho {} bytes", letra, std::mem::size_of_val(&letra));


    println!("Hello, world!");
}
