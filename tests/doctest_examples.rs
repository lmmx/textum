#![allow(missing_docs)]
#[cfg(feature = "json")]
#[test]
fn example() -> Result<(), textum::PatchError> {
    use textum::{Patch, PatchSet};

    let dry_run = true; // Change to false to actually write files

    let input = r#"[
      {"file": "tests/fixtures/sample.txt", "range": [0, 5], "replacement": "goodbye"}
    ]"#;

    let patches: Vec<Patch> = facet_json::from_str(input)?;

    let mut set = PatchSet::new();
    for patch in patches {
        set.add(patch);
    }

    match set.apply_to_files() {
        Ok(results) => {
            for (file, content) in results {
                if dry_run {
                    println!("Would write {content} to {file}");
                } else {
                    std::fs::write(&file, content)?;
                    println!("Wrote to {file}");
                }
            }
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
