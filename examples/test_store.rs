use irmin::*;

fn main() -> Result<(), Error> {
    let config = Config::<serde_json::Value>::git_mem()?;
    let repo = Repo::new(config)?;
    let mut store = Store::new(&repo)?;

    let info = repo.info("irmin", "set")?;
    let path = Path::from_str(&repo, "foo/bar")?;
    let value = serde_json::json!({
        "a": 1i64,
        "b": 2i64,
        "c": 3i64,
    });
    assert!(store.set(&path, &value, info)?);

    let head = store.head()?.unwrap();
    assert!(head.parents()?.len() == 0);

    let s = store.find(&path)?;
    assert!(s.unwrap() == value);

    let path1 = path.parent()?.unwrap();
    assert!(store.mem_tree(&path1));

    let x = store.find_tree(&path1)?;
    assert!(x.is_some());

    let path2 = repo.path(&["bar"])?;
    let y = x.unwrap().find(&path2)?;
    assert!(y.unwrap() == value);

    let value1 = serde_json::json!({
        "a": 4i64,
        "b": 5i64,
        "c": 6i64,
    });

    let info = Info::new(&repo, "irmin", "set")?;
    assert!(store.set(&path, &value1, info)?);

    let head1 = store.head()?.unwrap();
    assert!(head1.parents()?.len() == 1);
    assert!(head1.parents()?[0] == head);

    let tree = head1.tree().unwrap();
    assert!(
        tree == store
            .find_tree(&Path::empty(&repo).unwrap())
            .unwrap()
            .unwrap()
    );

    Ok(())
}