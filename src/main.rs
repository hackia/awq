use commit::AwqCommit;

pub mod ask;
pub mod commit;
pub mod pyamid;
fn main() -> Result<(), std::io::Error> {
    let mut commit: AwqCommit = AwqCommit::new();
    commit.save()
}
