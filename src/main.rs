use std::io;

// --- Atributos ---

trait CalculaArea {
    fn area(&self) -> f64;
}

// --- Structs isa---
#[derive(Debug)] 
struct Retangulo {
    largura: f64,
    altura: f64,
}

// --- impl isa---

impl Retangulo {
    // --- Construtor isa ---
    fn new(largura: f64, altura: f64) -> Result<Self, String> {
        if largura <= 0.0 || altura <= 0.0 {
            return Err("Dimensões devem ser positivas.".to_string());
        }
        Ok(Retangulo { largura, altura })
    }
}

// --- Polimorfismo e Herança (via Trait) val ---

impl CalculaArea for Retangulo {
    fn area(&self) -> f64 { self.largura * self.altura }
}

#[derive(Debug)]
struct Circulo {
    raio: f64,
}

impl Circulo {
    fn new(raio: f64) -> Result<Self, String> {
        if raio <= 0.0 {
            return Err("O raio deve ser positivo.".to_string());
        }
        Ok(Circulo { raio })
    }
}

impl CalculaArea for Circulo {
    fn area(&self) -> f64 { std::f64::consts::PI * self.raio.powi(2) }
}

#[derive(Debug)]
struct Triangulo {
    base: f64,
    altura: f64,
}

impl Triangulo {
    fn new(base: f64, altura: f64) -> Result<Self, String> {
        if base <= 0.0 || altura <= 0.0 {
            return Err("Base e altura devem ser positivas.".to_string());
        }
        Ok(Triangulo { base, altura })
    }
}

impl CalculaArea for Triangulo {
    fn area(&self) -> f64 { (self.base * self.altura) / 2.0 }
}

#[derive(Debug)]
struct Trapezio {
    base_maior: f64,
    base_menor: f64,
    altura: f64,
}

impl Trapezio {
    fn new(base_maior: f64, base_menor: f64, altura: f64) -> Result<Self, String> {
        if base_maior <= 0.0 || base_menor <= 0.0 || altura <= 0.0 {
            return Err("Todas as dimensões devem ser positivas.".to_string());
        }
        Ok(Trapezio { base_maior, base_menor, altura })
    }
}

impl CalculaArea for Trapezio {
    fn area(&self) -> f64 { ((self.base_maior + self.base_menor) * self.altura) / 2.0 }
}

#[derive(Debug)]
struct Paralelogramo {
    base: f64,
    altura: f64,
}

impl Paralelogramo {
    fn new(base: f64, altura: f64) -> Result<Self, String> {
        if base <= 0.0 || altura <= 0.0 {
            return Err("Base e altura devem ser positivas.".to_string());
        }
        Ok(Paralelogramo { base, altura })
    }
}

impl CalculaArea for Paralelogramo {
    fn area(&self) -> f64 { self.base * self.altura }
}

fn main() {
    println!("--- Calculadora de Áreas ---");

    loop {
        println!("\nEscolha a forma:");
        println!("1. Retângulo");
        println!("2. Círculo");
        println!("3. Triângulo");
        println!("4. Trapézio");
        println!("5. Paralelogramo");
        println!("6. Sair");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Falha ao ler a entrada.");

        match escolha.trim() {
            "1" => {
                println!("\n-- Retângulo --");
                let largura = ler_valor_numerico("Digite a largura:");
                let altura = ler_valor_numerico("Digite a altura:");

                // --- Tratamento de Exceções val---
                match Retangulo::new(largura, altura) {
                    // --- Instância criada com sucesso ---
                    Ok(forma) => {
                        // --- Polimorfismo val ---
                        println!("> A área é: {:.2}", forma.area());
                    },
                    Err(e) => println!("Erro: {}", e),
                }
            }
            "2" => {
                println!("\n-- Círculo --");
                let raio = ler_valor_numerico("Digite o raio:");

                match Circulo::new(raio) {
                    Ok(forma) => {
                        println!("> A área é: {:.2}", forma.area());
                    },
                    Err(e) => println!("Erro: {}", e),
                }
            }
            "3" => {
                println!("\n-- Triângulo --");
                let base = ler_valor_numerico("Digite a base:");
                let altura = ler_valor_numerico("Digite a altura:");

                match Triangulo::new(base, altura) {
                    Ok(forma) => {
                        println!("> A área é: {:.2}", forma.area());
                    },
                    Err(e) => println!("Erro: {}", e),
                }
            }
            "4" => {
                println!("\n-- Trapézio --");
                let base_maior = ler_valor_numerico("Digite a base maior:");
                let base_menor = ler_valor_numerico("Digite a base menor:");
                let altura = ler_valor_numerico("Digite a altura:");

                match Trapezio::new(base_maior, base_menor, altura) {
                    Ok(forma) => {
                        println!("> A área é: {:.2}", forma.area());
                    },
                    Err(e) => println!("Erro: {}", e),
                }
            }
            "5" => {
                println!("\n-- Paralelogramo --");
                let base = ler_valor_numerico("Digite a base:");
                let altura = ler_valor_numerico("Digite a altura:");

                match Paralelogramo::new(base, altura) {
                    Ok(forma) => {
                        println!("> A área é: {:.2}", forma.area());
                    },
                    Err(e) => println!("Erro: {}", e),
                }
            }
            "6" => {
                println!("Até a próxima!");
                break;
            }
            _ => {
                println!("Opção inválida! Por favor, escolha um número de 1 a 6.");
            }
        }
    }
}

// Função para ler valores numéricos da entrada do usuário
fn ler_valor_numerico(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a linha.");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Entrada inválida. Por favor, digite um número válido."),
        }
    }
}
