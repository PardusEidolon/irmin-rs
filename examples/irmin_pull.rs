use irmin::*;

fn main() -> Result<(), Error> {
    let _ = std::fs::remove_dir_all("/tmp/irmin-rs-test");
    let mut config = Config::<String>::git()?;
    config.set_root("/tmp/irmin-rs-test");
    let repo = Repo::new(config)?;
    let mut store = Store::new(&repo)?;
    let remote = Remote::url(&repo, "https://github.com/mirage/irmin-py")?;
    store.pull(&remote, None, None)?;
    assert!(store.mem(&Path::from_str(&repo, "README.md")?));
    Ok(())
}