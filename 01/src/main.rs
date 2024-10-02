fn main() {
    let mut linhas = std::io::stdin().lines();
    println!("Insira um numero para verificar se ele pertence a sequencia de Fibonacci:");
    let numero = linhas
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("A entrada nao eh um numero valido!");

    if is_fibonacci(numero) {
        println!("O numero {} pertence a sequencia de Fibonacci!", numero);
    } else {
        println!("O numero {} NAO pertence a sequencia de Fibonacci!", numero);
    }
}

// Verifica se o numero alvo pertence a sequencia de fibonacci
fn is_fibonacci(numero: u128) -> bool {
    // vetor contendo os primeiros dois numeros da sequencia de fibonacci
    let mut fibonacci = vec![0, 1];

    // gerar numeros da sequencia enquanto o numero alvo for maior que o ultimo
    while *fibonacci.last().unwrap() < numero {
        // soma os dois ultimos numeros da sequencia
        let proximo = fibonacci.last_chunk::<2>().unwrap().into_iter().sum();
        // adiciona o novo numero a sequencia
        fibonacci.push(proximo);
    }

    // exibir a sequencia calculada
    println!("Sequencia de fibonacci calculada: {:?}", fibonacci);

    // verificar se o numero alvo esta na sequencia
    let ultimo = *fibonacci.last().unwrap();
    match numero {
        n if n == ultimo => true, // basta verificar se o numero eh igual ao ultimo do vetor
        0 => true,                // o numero 0 eh o unico que nao vai ser o ultimo do vetor
        _ => false,               // nao esta na sequencia
    }
}

// Testes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_numeros_fibonacci() {
        // testando numeros que pertencem a sequencia de fibonacci
        assert!(is_fibonacci(0));
        assert!(is_fibonacci(1));
        assert!(is_fibonacci(2));
        assert!(is_fibonacci(3));
        assert!(is_fibonacci(5));
        assert!(is_fibonacci(8));
        assert!(is_fibonacci(13));
        assert!(is_fibonacci(21));
        assert!(is_fibonacci(34));
    }

    #[test]
    fn teste_numeros_nao_fibonacci() {
        // testando numeros que nao pertencem a sequencia de fibonacci
        assert!(!is_fibonacci(4));
        assert!(!is_fibonacci(6));
        assert!(!is_fibonacci(7));
        assert!(!is_fibonacci(9));
        assert!(!is_fibonacci(10));
        assert!(!is_fibonacci(15));
        assert!(!is_fibonacci(22));
    }

    #[test]
    fn teste_numero_grande_fibonacci() {
        // testando um numero de fibonacci grande (o 150º numero da sequencia)
        assert!(is_fibonacci(9969216677189303386214405760200));
    }

    #[test]
    fn teste_numero_grande_nao_fibonacci() {
        // testando um numero grande que nao pertence a sequencia de fibonacci
        assert!(!is_fibonacci(9969216677189303386214405760242));
    }
}
