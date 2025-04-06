use inquire::Editor;
use std::io::Error;

pub fn ask(question: &str) -> Result<String, Error> {
    Editor::new(question)
        .with_editor_command("vim".as_ref())
        .prompt()
        .map_err(|e| Error::new(std::io::ErrorKind::Other, e))
}

pub fn ask_commit_message() -> Result<String, Error> {
    ask("Enter commit message")
}

pub fn asked(question: &str) -> Result<String, Error> {
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
