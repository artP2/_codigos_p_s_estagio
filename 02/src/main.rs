fn main() {
    let mut linhas = std::io::stdin().lines();

    println!("Insira uma string para contar o numero de 'a's nela:");
    let string = linhas.next().unwrap().unwrap();

    let quantidade_a = conta_a(&string);

    match quantidade_a {
        0 => println!("A string nao possui nenhuma letra 'a'!"),
        1 => println!("A string possui 1 letra 'a'!"),
        _ => println!("A string possui {} letras 'a'!", quantidade_a),
    }
}

// retorna o numero de 'a's e 'A's na string
fn conta_a(string: &str) -> usize {
    // filtra os caracteres 'a' e 'A' e conta quantos sao
    string.chars().filter(|&c| c == 'a' || c == 'A').count()
}

// Testes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_strings_sem_a() {
        assert_eq!(conta_a("fiz bolo"), 0);
        assert_eq!(conta_a("comprei bolo"), 0);
    }

    #[test]
    fn teste_strings_com_a() {
        assert_eq!(conta_a("adoro bolo"), 1);
        assert_eq!(conta_a("comprei batata"), 3);
    }
}
