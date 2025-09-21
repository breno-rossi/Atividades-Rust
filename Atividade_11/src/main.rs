#[derive(Copy, Clone, Debug, PartialEq)]
struct Ponto {
    x: i32,
    y: i32,
}

fn trocar_copiavel<T: Copy>(a: &mut T, b: &mut T) {
    let temp = *a;  // Cria uma cópia do valor de 'a'
    *a = *b;        // Copia o valor de 'b' para 'a'
    *b = temp;      // Copia o valor temporário para 'b'
}

fn main() {
    println!("=== Teste com números inteiros (i32) ===");

    // Testando com i32 (tipo primitivo que já é Copy)
    let mut num1 = 10;
    let mut num2 = 20;

    println!("Antes da troca: num1 = {}, num2 = {}", num1, num2);
    trocar_copiavel(&mut num1, &mut num2);
    println!("Após a troca: num1 = {}, num2 = {}", num1, num2);
    assert_eq!(num1, 20);
    assert_eq!(num2, 10);
    println!("Teste com i32 passou!\n");


    println!("=== Teste com struct Ponto ===");
    let mut ponto1 = Ponto { x: 1, y: 2 };
    let mut ponto2 = Ponto { x: 3, y: 4 };
    println!("Antes da troca: ponto1 = {:?}, ponto2 = {:?}", ponto1, ponto2);
    trocar_copiavel(&mut ponto1, &mut ponto2);
    println!("Após a troca: ponto1 = {:?}, ponto2 = {:?}", ponto1, ponto2);
    assert_eq!(ponto1, Ponto { x: 3, y: 4 });
    assert_eq!(ponto2, Ponto { x: 1, y: 2 });
    println!("Teste com struct Ponto passou!\n");

    println!("=== Teste com outros tipos primitivos ===");

    // Testando com bool
    let mut bool1 = true;
    let mut bool2 = false;

    println!("Antes da troca: bool1 = {}, bool2 = {}", bool1, bool2);
    trocar_copiavel(&mut bool1, &mut bool2);
    println!("Após a troca: bool1 = {}, bool2 = {}", bool1, bool2);

    assert_eq!(bool1, false);
    assert_eq!(bool2, true);
    println!("Teste com bool passou!\n");

    // Testando com f64
    let mut float1 = 3.14;
    let mut float2 = 2.71;

    println!("Antes da troca: float1 = {}, float2 = {}", float1, float2);
    trocar_copiavel(&mut float1, &mut float2);
    println!("Após a troca: float1 = {}, float2 = {}", float1, float2);

    assert_eq!(float1, 2.71);
    assert_eq!(float2, 3.14);
    println!("Teste com f64 passou!\n");


    let mut str1 = String::from("Hello");
    let mut str2 = String::from("World");


    // trocar_copiavel(&mut str1, &mut str2);
    // 
    // Erro: the trait bound `String: Copy` is not satisfied


}
