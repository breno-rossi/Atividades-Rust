fn primeira_palavra(palavra: &str) -> &str{

    // Se começar com espaço retorna string vazia
    if palavra.starts_with(' ') {
        return "";
    }

    // Se a string estiver vazia, retorna vazia
    if palavra.is_empty() {
        return "";
    }

    for (i,c) in palavra.char_indices() {
        if c == ' ' {                     //Comparando o caracter em c com a string vazia ' '
            return &palavra[0..i];
        }
    }
    palavra
}

fn main() {
    let vetor_teste =
        ["hello world",
            " rust",
            "rust",
            "",
            " ",
            "bomdiatudobem?"];

    for p in vetor_teste {
        println!("1ºPalavra: {}", primeira_palavra(p))
    };
}
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
    fn my_tests() {
        assert_eq!(primeira_palavra("hello world"), "hello");
        assert_eq!(primeira_palavra(" rust"), "");
        assert_eq!(primeira_palavra("rust"), "rust");
        assert_eq!(primeira_palavra(""), "");
        assert_eq!(primeira_palavra("   "), "");
        assert_eq!(primeira_palavra("bomdiatudobem?"), "bomdiatudobem?");
    } }
