# Guía Intensiva de Rust

## Día 1: Instalación y Fundamentos de Rust en Smart Contracts

### Objetivos:

- Instalar Rust y herramientas esenciales.
- Comprender los conceptos básicos del lenguaje.
- Escribir un primer smart contract simple.

---

## 1. Instalación y Configuración

### Instalar Rust con rustup:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Instalar dependencias para Rust en Smart Contracts:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --feature opt
```

### Configurar un nuevo proyecto Rust:

```sh
cargo new mi_primer_contrato
cd mi_primer_contrato
```

[Guia Oficial para Smart Contracts](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)

### 2. Fundamentos de Rust

🔹 Sintaxis Básica

Rust es un lenguaje tipado estáticamente, lo que significa que los tipos de las variables se conocen en tiempo de compilación. Algunas características clave son:
📌 Variables y Mutabilidad:

En Rust, las variables son inmutables por defecto. Para hacerlas mutables, se usa mut.

```rs
let x = 5; // Variable inmutable
let mut y = 10; // Variable mutable
y += 5;
println!("El valor de y es: {}", y);
```

### 📌 Tipos de Datos:

Rust tiene tipos primitivos bien definidos:

- Enteros (i8, i16, i32, i64, u8, u16, u32, u64)
- Booleanos (bool)
- Caracteres (char) // Existe pero no se utiliza demasiado.
- Tuplas ((i32, f64, char)) // Uso limitado.
- Arreglos ([i32; 3]) // Acceso por índice [0].

```rs
let entero: i32 = 42;
let flotante: f64 = 3.14;
let booleano: bool = true;
let caracter: char = 'R';
let tupla: (i32, f64, char) = (10, 2.5, 'A');
let arreglo: [i32; 3] = [1, 2, 3];
```

🔹 Funciones y Ownership

En Rust, las funciones se declaran con fn y pueden devolver valores usando ->.

```rs
fn suma(a: i32, b: i32) -> i32 {
    a + b
}
```

### 📌 Ownership (Propiedad):

Rust maneja la memoria con ownership en lugar de garbage collection.

Reglas de Ownership:

- Cada valor en Rust tiene un "dueño".
- Solo puede haber un dueño a la vez.
- Cuando el dueño sale del ámbito, el valor se libera.

```rs
fn main() {
    let s1 = String::from("Hola");
    let s2 = s1; // s1 pierde su "ownership" y no se puede usar
    println!("{}", s2);
}
```

Para clonar una variable y evitar perderla:

```rs
let s1 = String::from("Hola");
let s2 = s1.clone();
println!("{} y {}", s1, s2);
```

### 🔹 Estructuras de Control

#### 📌 Condicionales (if):

```rs
let numero = 10;
if numero > 5 {
    println!("El número es mayor que 5");
} else {
    println!("El número es 5 o menor");
}
```

#### 📌 Bucles (loop, while, for):

```rs
let mut contador = 0;
while contador < 5 {
    println!("Contador: {}", contador);
    contador += 1;
}

for i in 0..5 {
    println!("Iteración: {}", i);
}

```

### 3. Primer Smart Contract en Rust (Soroban)

#### Crear un smart contract básico en Soroban:

```rs
#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct MiContrato;

#[contractimpl]
impl MiContrato {
    pub fn hola(env: Env) -> str {
        "Hola, mundo!"
    }
}

```

## 🏆 Actividades y Tareas

### ✅ Escribir un programa Rust que reciba dos números y devuelva su suma.

### ✅ Crear un smart contract que devuelva un mensaje diferente según la entrada.

### 🎯 Actividades para la Casa

#### 🔹 Tarea 1: Smart Contract - Verificación de Números Primos

📌 Descripción:
Escribe un Smart Contract que reciba un número entero y determine si es primo.

📌 Requisitos:

- Si el número es negativo, debe retornar 0.
- Si el número es primo, debe retornar:
- "El número X es primo."
- Si el número no es primo, debe retornar:
- "El número X no es primo."

#### 🔹 Tarea Avanzada:

- Si el número no es primo, imprimir sus múltiplos.

#### Tips:

- Para armar un build del codigo que estan escribiendo asegurense de que el terminal que tienen abierto este dentro de la carpeta del proyecto.
- Para compilar el codigo pueden correr el siguiente comando

```sh
cargo build
```

- Para correr el codigo pueden correr el siguiente comando

```sh
cargo run
```
