#![allow(unused)]

use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;

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

                Ok((mut stream, address)) => {
                    // Para ler as informações da conexão, criamos um array que armazenará até 1024 bytes de dados
                    // Esse tamnho foi proposto só para teste, em um servidor real, deveríamos alocar um espaço maior
                    // para evitar perder dados.

                    let mut buffer = [0; 1024];
                    
                    // Assim, colocamos um pointer para o buffer, informando que ele pode ser alterado pelo stream.read()

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // A função try_from espera receber um array [u8], porém ela está recebendo um [u8, 1024], por isso temos que fazer um slice dele
                            match Request::try_from(&buffer[..]){
                                Ok(_) => {},
                                Err(e) => println!("{}", e)
                            } 
                        }
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                },

                Err(e) => println!("Failed to estabilish a connection: {}", e)
            }


        }
    }
}
