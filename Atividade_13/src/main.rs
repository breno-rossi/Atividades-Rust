use num_traits::FromPrimitive;
use std::fmt::Display;
use std::ops::{Add, Mul};
use std::cmp::PartialOrd;

// 1. Trait genérico Calculavel<T>
trait Calculavel<T> {
    fn area(&self) -> T;
    fn perimetro(&self) -> T;
}

// 2. Estruturas genéricas para formas geométricas
#[derive(Debug)]
struct Circulo<T> {
    raio: T,
}

#[derive(Debug)]
struct Retangulo<T> {
    largura: T,
    altura: T,
}

// Implementação de construtores com validação
impl<T> Circulo<T>
where
    T: Copy + FromPrimitive + PartialOrd,
{
    fn new(raio: T) -> Result<Self, String> {
        let zero = T::from_f64(0.0).expect("Falha ao converter 0.0 para T");
        if raio >= zero {
            Ok(Circulo { raio })
        } else {
            Err("O raio deve ser não-negativo".to_string())
        }
    }
}

impl<T> Retangulo<T>
where
    T: Copy + FromPrimitive + PartialOrd,
{
    fn new(largura: T, altura: T) -> Result<Self, String> {
        let zero = T::from_f64(0.0).expect("Falha ao converter 0.0 para T");
        if largura >= zero && altura >= zero {
            Ok(Retangulo { largura, altura })
        } else {
            Err("A largura e altura devem ser não-negativas".to_string())
        }
    }
}

// 3. Implementação do trait Calculavel<T> para Circulo<T>
impl<T> Calculavel<T> for Circulo<T>
where
    T: Copy + Mul<Output = T> + FromPrimitive,
{
    fn area(&self) -> T {
        // Área: π * raio²
        let pi_t = T::from_f64(std::f64::consts::PI).expect("Falha ao converter PI para T");
        pi_t * self.raio * self.raio
    }

    fn perimetro(&self) -> T {
        // Perímetro: 2 * π * raio
        let pi_t = T::from_f64(std::f64::consts::PI).expect("Falha ao converter PI para T");
        let dois_t = T::from_f64(2.0).expect("Falha ao converter 2.0 para T");
        dois_t * pi_t * self.raio
    }
}

// Implementação do trait Calculavel<T> para Retangulo<T>
impl<T> Calculavel<T> for Retangulo<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + FromPrimitive,
{
    fn area(&self) -> T {
        // Área: largura * altura
        self.largura * self.altura
    }

    fn perimetro(&self) -> T {
        // Perímetro: 2 * (largura + altura)
        let dois_t = T::from_f64(2.0).expect("Falha ao converter 2.0 para T");
        dois_t * (self.largura + self.altura)
    }
}

fn main() {
    println!("=== Sistema de Formas Geométricas Genéricas ===\n");

    // Testando com f64
    println!("--- Testando com f64 ---");

    // Círculo com f64
    match Circulo::new(5.0f64) {
        Ok(circulo) => {
            println!("Círculo de raio {:.2}:", circulo.raio);
            println!("Área: {:.2}", circulo.area());
            println!("Perímetro: {:.2}", circulo.perimetro());
        }
        Err(e) => println!("Erro ao criar círculo: {}", e),
    }

    println!();

    // Retângulo com f64
    match Retangulo::new(4.0f64, 6.0f64) {
        Ok(retangulo) => {
            println!("Retângulo de largura {:.2} e altura {:.2}:", retangulo.largura, retangulo.altura);
            println!("Área: {:.2}", retangulo.area());
            println!("Perímetro: {:.2}", retangulo.perimetro());
        }
        Err(e) => println!("Erro ao criar retângulo: {}", e),
    }

    println!("\n--- Testando com f32 ---");

    // Círculo com f32
    match Circulo::new(3.5f32) {
        Ok(circulo) => {
            println!("Círculo de raio {:.2}:", circulo.raio);
            println!("Área: {:.2}", circulo.area());
            println!("Perímetro: {:.2}", circulo.perimetro());
        }
        Err(e) => println!("Erro ao criar círculo: {}", e),
    }

    println!();

    // Retângulo com f32
    match Retangulo::new(7.5f32, 2.0f32) {
        Ok(retangulo) => {
            println!("Retângulo de largura {:.2} e altura {:.2}:", retangulo.largura, retangulo.altura);
            println!("Área: {:.2}", retangulo.area());
            println!("Perímetro: {:.2}", retangulo.perimetro());
        }
        Err(e) => println!("Erro ao criar retângulo: {}", e),
    }

    println!("\n--- Testando validação de valores inválidos ---");

    // Testando valores negativos
    match Circulo::new(-2.0f64) {
        Ok(_) => println!("Círculo criado com sucesso"),
        Err(e) => println!("Erro esperado: {}", e),
    }

    match Retangulo::new(-1.0f32, 5.0f32) {
        Ok(_) => println!("Retângulo criado com sucesso"),
        Err(e) => println!("Erro esperado: {}", e),
    }

    println!("\n--- Casos especiais ---");

    // Círculo unitário
    match Circulo::new(1.0f64) {
        Ok(circulo) => {
            println!("Círculo unitário (raio = 1.00):");
            println!("Área: {:.2} (≈ π)", circulo.area());
            println!("Perímetro: {:.2} (≈ 2π)", circulo.perimetro());
        }
        Err(e) => println!("Erro: {}", e),
    }

    println!();

    // Quadrado (retângulo com lados iguais)
    match Retangulo::new(5.0f32, 5.0f32) {
        Ok(quadrado) => {
            println!("Quadrado de lado {:.2}:", quadrado.largura);
            println!("Área: {:.2}", quadrado.area());
            println!("Perímetro: {:.2}", quadrado.perimetro());
        }
        Err(e) => println!("Erro: {}", e),
    }
}