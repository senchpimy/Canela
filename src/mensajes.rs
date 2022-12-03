use std::fs;

pub fn enviar(texto:&mut String ){
    if texto.contains("\n"){
        println!("AAAAA");
        encriptar(texto);
        *texto=String::new();
    }
}


pub struct Mensaje{
    pub usuario:String,
    pub contenido:String,
    //binario:String
}

fn encriptar(texto:&mut String){
let texto_reves:String = texto.chars().rev().collect();
println!("{}",texto_reves.trim());
}

//fn leer_mensajes()->String{
//let contents = fs::read_to_string("mensajes")
//    .expect("Leyendo mensajes");
//    contents
//}

pub fn obtener_mensajes() -> Vec<Mensaje>{
    let mut mensjaes:Vec<Mensaje>=Vec::new();
    for i in 0..5{
        mensjaes.push(Mensaje{usuario:String::from(format!("usuario no. {}",i)), 
            contenido:String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")});
    }
    mensjaes

}

