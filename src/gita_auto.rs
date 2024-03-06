use std::process::Command;
use std::str;

use crate::diff::diff::git_diff;
use crate::utils::utility::{add_commit, print_line, print_text_auto_commit};
use colored::Colorize;
//puede recibir un array de strings que representan los tipos de archivos a comitear

pub fn gita_auto(file_types: Vec<&str>) {
    let git_status_output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute git status");

    if !git_status_output.status.success() {
        println!(
            "{}",
            "Failed to execute git status, you are in repository?".bright_red()
        );
        return;
    }
    let output_str =
        str::from_utf8(&git_status_output.stdout).expect("Error al convertir la salida a cadena");

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
            "{} ",
            "Changes not staged for commit: Auto commiting files"
                .bold()
                .bright_purple(),
        );
        if file_types.len() > 0 {
            let splice_types = file_types.iter().position(|&x| x.contains("-t")).unwrap();
            let file_types = &file_types[splice_types + 1..];
            // ...

            print!("{:?}", file_types);
        }
        let splice_index = lines
            .iter()
            .position(|&x| x.contains("Changes not staged for commit:"))
            .unwrap();
        let lines = &lines[splice_index..];

        for line in lines {
            if line.contains("modified:") {
                let file_to_add = line.split_whitespace().last().unwrap();

                let msg_commit = git_diff(file_to_add);
                print_text_auto_commit(file_to_add, "MODIFIED", &msg_commit);

                add_commit("MODIFIED", file_to_add, &msg_commit);
                print_line();
            } else if line.contains("deleted:") {
                let file_to_add = line.split_whitespace().last().unwrap();
                let msg_commit = git_diff(file_to_add);

                print_text_auto_commit(file_to_add, "DELETED", &msg_commit);

                add_commit("DELETED", file_to_add, &msg_commit);

                print_line();
            } else if line.contains("new file:") {
                let file_to_add = line.split_whitespace().last().unwrap();
                let msg_commit = git_diff(file_to_add);

                print_text_auto_commit(file_to_add, "NEW", &msg_commit);

                add_commit("NEW", file_to_add, &msg_commit);

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
                    let msg_commit = git_diff(file_to_add);

                    print_text_auto_commit(file_to_add, "ADD", &msg_commit);

                    add_commit("[ADD]", file_to_add, &msg_commit);
                    print_line();
                }
                break;
            }
        }
    }
    //d
    print!(
        "{}",
        "Auto commit done, push to the branch. ".bright_magenta()
    )
}
