pub mod utility {
    use colored::Colorize;
    use std::process::Command;
    use std::str;

    pub fn print_text_commit(file_to_add: &str, ctype: &str) {
        println!(
            "\nFile {}: {}\n{} commit -m {}{}{}{}{} \n{}or{}\n",
            ctype.bright_white(),
            file_to_add.bright_cyan(),
            "git".bright_yellow(),
            format!("\"[{}] @", ctype).bright_blue(),
            file_to_add.bright_blue(),
            " ^new^ ".bright_blue(),
            "#yourmessage".bright_green(),
            "\"".bright_blue(),
            "type commit msg ".bright_green(),
            " press enter to omit".bright_red()
        );
    }
    pub fn add_commit(ctype: &str, file_to_add: &str, input: &str) {
        Command::new("git")
            .arg("add")
            .arg(file_to_add)
            .output()
            .expect("Error al ejecutar git add");
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(format!(
                "[{}] @{} #{}",
                ctype,
                file_to_add,
                input.trim()
            ))
            .output()
            .expect("Error al ejecutar git commit");

        println!(
            "{}{}",
            "File commit -> ".bright_green(),
            file_to_add.bright_cyan()
        );
    }
    pub fn print_omited_file(file_to_add: &str) {
        println!("Archivo omitido {}", file_to_add.bright_red());
    }
    pub fn print_line() {
        print!(
            "{}",
            "\n------------------------------------------------\n".bright_magenta()
        )
    }
    pub fn print_text_auto_commit(file_to_add: &str, ctype: &str, message: &str) {
        println!(
            "\nFile {} -> {}\n{} commit -m {}{} {} {}",
            ctype.bright_white(),
            file_to_add.bright_cyan(),
            "git".bright_yellow(),
            format!("\"[{}] @", ctype).bright_blue(),
            file_to_add.bright_blue(),
            message.bright_blue(),
            "\"".bright_blue(),
        );
    }
}
