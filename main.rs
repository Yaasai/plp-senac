use std::collections::HashMap;

/*
 * Software em Rust que conta a quantidade de linhas e palavras (dividido por espaço).
 * Por padrão o o sotware apenas lê o arquivo texto. 
 * –l só exibe a quantidade de linhas
 * –w só exibe a quantidade de palavras
 */

struct Cli {
  pattern: String,
  path: std::path::PathBuf,
}
type Callback = fn(String);

fn contar_linhas(conteudo: String) {
    let linhas = conteudo.lines();
    println!("O arquivo possui {} linhas", linhas.count());
}

fn contar_palavras(conteudo: String) {
    let linhas = conteudo.lines();
    let mut contador: usize = 0;
    
    for linha in linhas {
        let palavras: Vec<&str> = linha.split(" ").collect();
        contador += palavras.len();
    }

    println!("O arquivo possui {} palavras", contador);
}

fn mostrar_texto(conteudo: String) {
    let linhas = conteudo.lines();
    for line in linhas {
        println!("{}", line);
    }
}

fn obter_opcoes_validas() -> HashMap<String, Callback> {
    let mut opcoes_validas: HashMap<String, Callback> =  HashMap::new(); 

    opcoes_validas.insert("-W".to_string(), contar_palavras);
    opcoes_validas.insert("-L".to_string(), contar_linhas);
    opcoes_validas.insert("".to_string(), mostrar_texto);

    return opcoes_validas;
}

fn help() {
    println!("Software em Rust que conta a quantidade de linhas e palavras (dividido por espaço).\n");
    println!("\tPor padrão o o sotware apenas lê o arquivo texto.");
    println!("\t–l só exibe a quantidade de linhas");
    println!("\t–w só exibe a quantidade de palavras\n");
}

fn ler_args(args: Vec<String>, opcoes_validas: &HashMap<String, Callback>) -> Cli {
    let cli;

    match args.len() {

        // Por padrão o o sotware apenas lê o arquivo texto. 
        2 => {
            let path = &args[1]; 

            cli = Cli {
                pattern: "".to_string(),
                path: std::path::PathBuf::from(path),
            };
        },

        // –l só exibe a linha 
        // –w só exibe a quantidade de palavras
        3 => {
            let path = &args[2]; 
            let pattern = String::from(&args[1]);

            if !(opcoes_validas.contains_key(&pattern.to_uppercase())) {
                println!("Opção {} é inválida!", pattern);
                help();
                panic!()
            }

            cli = Cli {
                pattern: pattern,
                path: std::path::PathBuf::from(path),
            };
        },

        _ => {
            println!("Opção inválida!");
            help();
            panic!()
        }
    }

    return cli;
}


fn main() {
    let opcoes_validas = obter_opcoes_validas();
    let cli = ler_args(std::env::args().collect(), &opcoes_validas);

    let conteudo = std::fs::read_to_string(&cli.path)
    .expect("Arquivo não localizado.");

    opcoes_validas[&cli.pattern.to_uppercase()](conteudo);
}

