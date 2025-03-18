fn main() {
    // ===== Tipos de Datos =====

    // Enteros
    let numero_entero: i32 = 42;
    let numero_unsigned: u8 = 255;

    // Flotantes
    let numero_flotante: f64 = 3.14;

    // Booleanos
    let es_rust_genial: bool = true;

    // Caracteres
    let letra: char = 'R';

    // Tuplas (pueden contener distintos tipos)
    let tupla: (i32, f64, char) = (10, 2.5, 'A');
    println!("Primer elemento de la tupla: {}", tupla.0);

    // Arreglos (de longitud fija)
    let arreglo: [i32; 3] = [1, 2, 3];
    println!("Primer elemento del arreglo: {}", arreglo[0]);

    // ===== Estructuras de Control =====

    // Condicional if
    let edad = 18;
    if edad >= 18 {
        println!("Eres mayor de edad.");
    } else {
        println!("Eres menor de edad.");
    }

    // Bucle while
    let mut contador = 0;
    while contador < 3 {
        println!("Contador: {}", contador);
        contador += 1;
    }

    // Bucle for con rango
    for i in 1..=5 {
        println!("Iteración: {}", i);
    }

    // ===== Funciones (Ejemplos sin resolución) =====

    // Función sin parámetros
    fn mensaje() {
        println!("Este es un mensaje desde una función.");
    }

    // Función con parámetros
    fn mostrar_numero(n: i32) {
        println!("El número es: {}", n);
    }

    // Función con retorno implicito
    fn obtener_valor() -> i32 {
        10
    }

    // Función con retorno explicito
    fn obtener_valor2() -> i32 {
        return 10;
    }

    // Llamando a las funciones
    mensaje();
    mostrar_numero(5);
    let valor = obtener_valor();
    println!("Valor retornado: {}", valor);

    let valor2 = obtener_valor2();
    println!("Valor retornado: {}", valor2);


}
