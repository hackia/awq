use commit::AwqCommit;
use std::process::exit;
pub mod ask;
pub mod commit;
pub mod run;
fn main() {
    match AwqCommit::new() {
        Ok(mut app) => {
            if let Err(e) = app.save() {
                eprintln!("{e}");
                exit(1)
            }
            exit(0)
        }
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    }
}
