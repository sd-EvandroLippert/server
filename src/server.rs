#![allow(unused)]

use std::net::TcpListener;

// Em Rust a criação de classes é diferente;
// Struct serve para passar as variáveis;
// Impl serve para descrever as funções e métodos
pub struct Server {
    address: String,
}


impl Server {

    // Fn sem self é tipo o static method em python
    pub fn new(address: String) -> Self {  // Self aqui serve para dizer que a função retornará uma Struct de Server
        Self {
            address: address,
        }
    }

    // Aqui usamos o & antes do self, para que ele pegue emprestada a variável e 
    // após executar o run, não delete a estrutura.
    pub fn run(&self) {
        // O TcpListener:: bind retorna um elemento do tipo Result;
        // Esse tipo serve para indicar que a função pode retornar dois resultados;
        // Um correto ou um erro;
        // Precisamos user o atributo unwrap() para desembrulhar o resultado e atribuir a nossa variável.

        let listener: TcpListener = TcpListener::bind(&self.address).unwrap();
        
        // Para realizar um loop infinito em rust nós podemos usar a função loop
        loop {
            // match é uma forma de aplicar condições:
            // Ou seja se listener.accept() retornar ok, entrará na primeira condição;
            // como retorna uma tupla, stream receberá o primeiro valor e address o segundo
            // Entretanto, caso retorne Err (erro), ele entrará na segunda condição
            // que é printar a mensagem na tela
            match listener.accept() {

                Ok((stream, address)) => {
                    println!("Ok!")
                },

                Err(e) => println!("Failed to estabilish a connection: {}", e)
            }


        }
    }
}
