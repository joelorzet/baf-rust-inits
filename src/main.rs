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

    fn factorizar_numero(numero_a_factorizar: i128) -> String {
        let mut multiplos: Vec<i128> = Vec::new();

        for numero_actual in 2..=numero_a_factorizar - 1 {
            if numero_a_factorizar % numero_actual == 0 {
                multiplos.push(numero_actual);
            }
        }

        return format!("Los multiplos de {} son: {:?}", numero_a_factorizar, multiplos);
    }

    fn es_numero_primo(numero_a_verificar: i128) -> String {
        if numero_a_verificar <= 0 {
            return "0".to_string();
        }

        let mut es_primo = true;

        for divisor in 2..=numero_a_verificar - 1 {
            let resto = numero_a_verificar % divisor;

            if resto == 0 {
                es_primo = false;
                break;
            }
        }

        if es_primo {
            return format!("El numero {} es primo", numero_a_verificar);
        } else {
            factorizar_numero(numero_a_verificar)
        }
    }

    let es_primo = es_numero_primo(4);

    println!("{}", es_primo);
}
