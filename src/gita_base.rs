use colored::Colorize;
use std::process::Command;
use std::str;

use crate::utils::utility::{add_commit, print_line, print_omited_file, print_text_commit};

pub fn gita_base() {
    let git_status_output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute git status");

    if !git_status_output.status.success() {
        println!("{}","Failed to execute git status, you are in repository?".bright_red());
    }
    // Obtener la salida como cadena UTF-8
    let output_str =
        str::from_utf8(&git_status_output.stdout).expect("Error al convertir la salida a cadena");

    // Dividir la salida en líneas
    let lines: Vec<&str> = output_str.lines().collect();

    if lines
        .iter()
        .any(|&x| x.starts_with("nothing to commit, working tree clean"))
    {
        print!("{}", "No changes to add and commit ^-^".bright_purple());
        return;
    } else if lines
        .iter()
        .any(|&x| x.starts_with("Changes not staged for commit:"))
    {
        print_line();
        println!(
            "{}",
            "Changes not staged for commit: Type msg to add & commit"
                .bold()
                .bright_purple()
        );
        let splice_index = lines
            .iter()
            .position(|&x| x.contains("Changes not staged for commit:"))
            .unwrap();
        let lines = &lines[splice_index..];

        for line in lines {
            if line.contains("modified:") {
                let file_to_add = line.split_whitespace().last().unwrap();

                print_text_commit(file_to_add, "MODIFIED");

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                if input.trim() != "" {
                    add_commit("MODIFIED", file_to_add, &input);
                } else {
                    print_omited_file(file_to_add);
                }
                print_line();
            } else if line.contains("deleted:") {
                let file_to_add = line.split_whitespace().last().unwrap();

                print_text_commit(file_to_add, "DELETED");

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                if input.trim() != "" {
                    add_commit("DELETED", file_to_add, &input);
                } else {
                    print_omited_file(file_to_add);
                }
                print_line();
            } else if line.contains("Untracked files:") {
                println!("Hay archivos no rastreados");
                let splice_index = lines
                    .iter()
                    .position(|&x| x.contains("Untracked files:"))
                    .unwrap();
                let lines = &lines[splice_index + 2..];

                for line in lines {
                    if line.is_empty() {
                        break;
                    }
                    let file_to_add = line.split_whitespace().last().unwrap();
                    print_text_commit(file_to_add, "ADD");

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();

                    if input.trim() != "" {
                        add_commit("ADD", file_to_add, &input);
                    } else {
                        print_omited_file(file_to_add);
                    }
                    print_line();
                }
                break;
            }
        }
        print!(
            "{}",
            "No more files to add and commit, push to the branch.".bright_magenta()
        );
        return;
    }
}
