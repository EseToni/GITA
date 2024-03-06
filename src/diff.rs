pub mod diff {

    use std::process::Command;
    use std::str;

    pub fn git_diff(file: &str) -> String {
        let git_diff_output = Command::new("git")
            .arg("diff")
            .arg("--numstat")
            .arg(file)
            .output()
            .expect("failed to execute git diff");

        if !git_diff_output.status.success() {
            println!("{}", "Failed to execute git diff, you are in repository?");
            return "Error".to_string();
        }
        let output_str =
            str::from_utf8(&git_diff_output.stdout).expect("Error al convertir la salida a cadena");
        output_str.to_string();

        // Lines to Vec<&str>
        let lines: Vec<&str> = output_str.lines().collect();
        let numbers: Vec<&str> = lines[0].split_whitespace().collect();
        // Get insertions and deletions
        let insertions = numbers[0];
        let deletions = numbers[1];
        // convert to i32
        let insertions = insertions.parse::<i32>().unwrap();
        let deletions = deletions.parse::<i32>().unwrap();
        // Total changes
        let total = insertions - deletions;

        if (insertions > 50 || deletions > 50) && (total < 20 || total > -20) {
            return "Refactor code".to_string();
        } else if (insertions > 20 || deletions > 20) && (total < 20 || total > -20) {
            return "Code improvement".to_string();
        } else if (insertions > 10 || deletions > 10) && (total < 20 || total > -20) {
            return "Small optimize".to_string();
        } else if total > 50 {
            return "Add new feature".to_string();
        } else if total > 20 {
            return "Add new function".to_string();
        } else if total > 10 {
            return "Add small improve".to_string();
        } else if total > 5 {
            return "Add minimal improve".to_string();
        } else if total < -50 {
            return "Remove feature".to_string();
        } else if total < -20 {
            return "Remove function".to_string();
        } else if total < -10 {
            return "Remove small code".to_string();
        } else {
            return "miminal changes".to_string();
        }
    }
}
