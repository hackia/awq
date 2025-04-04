use inquire::Editor;

pub fn ask(question: &str) -> Result<String, std::io::Error> {
    Editor::new(question)
        .with_editor_command(&"vim".as_ref())
        .prompt()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

pub fn ask_commit_message() -> Result<String, std::io::Error> {
    ask("Enter commit message")
}

pub fn asked(question: &str) -> Result<String, std::io::Error> {
    loop {
        let q = ask(question);
        match q {
            Ok(answer) => {
                if answer.is_empty() {
                    println!("Please provide a non-empty answer.");
                } else {
                    return Ok(answer);
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}
