use std::process::Command;

// Função para executar um comando com os argumentos especificados
fn execute_command(command: &str, args: &[&str]) {
    println!("Executando: {} {}", command, args.join(" "));
    
    // Cria e executa o comando
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Falha ao executar o comando");

    // Verifica se o comando foi executado com sucesso
    if output.status.success() {
        println!("Comando '{}' executado com sucesso!", command);
        println!("Saída:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Erro ao executar o comando '{}':", command);
        eprintln!("Detalhes:\n{}", String::from_utf8_lossy(&output.stderr));
    }
    println!("---------------------------------");
}

fn main() {
    // Executa 'sudo pacman -Syu'
    execute_command("sudo", &["pacman", "-Syu"]);

    // Executa 'yay -syu'
    execute_command("yay", &["-syu"]);

    // Executa 'flatpak update'
    execute_command("flatpak", &["update"]);
}
