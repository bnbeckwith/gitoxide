use crate::{check_common, create_repo, inspect_refs};
use git_commitgraph::Graph;

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let repo_dir = create_repo("two_parents.sh");
    let refs = inspect_refs(repo_dir.path(), &["parent1", "parent2", "child"]);
    let cg = Graph::from_info_dir(repo_dir.path().join(".git").join("objects").join("info"))?;
    check_common(&cg, &refs);

    assert_eq!(cg.commit_at(refs["parent1"].pos()).generation(), 1);
    assert_eq!(cg.commit_at(refs["parent2"].pos()).generation(), 1);
    assert_eq!(cg.commit_at(refs["child"].pos()).generation(), 2);

    Ok(())
}