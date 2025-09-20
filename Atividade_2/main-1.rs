fn main() {

    let w1= String::from("hello world");
    let w2= String::from("rust");
    let w3= String::from("Atenção: aqui tem uma pegadinha!!!");

    let indice1 = indice_final_primeira_palavra(&w1);
    let indice2 = indice_final_primeira_palavra(&w2);
    let indice3 = indice_final_primeira_palavra(&w3);

    println!("A primeira palavra de '{w1}' termina com o indice {indice1}");
    println!("A primeira palavra de '{w2}' termina com o indice {indice2}");
    println!("A primeira palavra de '{w3}' termina com o indice {indice3}");

    /*Ao printar a w3 notamos que a palavra "Atenção:" termina com o indice 10, contudo,
     quando contamos "manualmente" a palavra apenas conseguimos identificar 8 indices.
     Isso ocorre devido a codificação UTF-8 que armazena os caracteres 'ç' e 'ã' sendo de 2 bytes e os demais
     1 byte apenas
     */

fn indice_final_primeira_palavra(palavra_recebida: &String) -> usize{

    let palavra_bytes = palavra_recebida.as_bytes();

    for (i,b) in palavra_bytes.iter().enumerate()
    {
        if *b == b' '
        {
            return i;   //Caso encontre espaço em branco retorna o indice
        }
    }
    palavra_recebida.len() //Se nao encontrar espaço em branco retorna o comprimento da palavra

    }
}
