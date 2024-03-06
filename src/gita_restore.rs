use std::process::Command;
use std::str;
use std::time::Duration;

pub fn gita_restore() {
    let git_status_output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute git status");

    if git_status_output.status.success() {
        let output_str = str::from_utf8(&git_status_output.stdout)
            .expect("Error al convertir la salida a cadena");
        let lines: Vec<&str> = output_str.lines().collect();

        if lines
            .iter()
            .any(|&x| x.starts_with("nothing to commit, working tree clean"))
        {
            print!("No hay cambios pendientes");
            return;
        } else if lines
            .iter()
            .any(|&x| x.starts_with("Changes to be committed:"))
        {
            println!("Hay cambios commiteados");
            let splice_index = lines
                .iter()
                .position(|&x| x.contains("Changes to be committed:"))
                .unwrap();
            let lines = &lines[splice_index..];

            for line in lines {
                if line.contains("new file:")
                    || line.contains("modified:")
                    || line.contains("deleted:")
                {
                    let file_to_add = line.split_whitespace().last().unwrap();
                    println!(
                        "\nStaged file: {}\n¿Quieres restaurarlo?(s/n)\n",
                        file_to_add
                    );
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();

                    if input.trim() == "s" {
                        Command::new("git")
                            .arg("restore")
                            .arg("--staged")
                            .arg(file_to_add)
                            .output()
                            .expect("Error al ejecutar git restore");
                        println!("Archivo restaurado {}", file_to_add);
                        
                        std::thread::sleep(Duration::from_secs(1));
                    } else {
                        println!("file omitted");
                    }
                }
            }
        }
    }
}
