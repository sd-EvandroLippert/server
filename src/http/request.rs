#![allow(unused)]

use super::method::Method;

pub struct Request {
    

    path: String,
    query_string: Option<String>,  // Option<> Serve para dizer que esse parâmetro é opcional
    method: Method,
}
