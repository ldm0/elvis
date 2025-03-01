use etc::{Etc, FileSystem, Tree, Write};
use std::{env, path::PathBuf};

fn main() {
    let mut example = env::current_dir().unwrap();
    example.push("../../examples/hello-world");
    assert!(example.exists());

    // Remove pkg directory
    let mut pkg = example.clone();
    pkg.push("pkg");
    if pkg.exists() {
        Etc::from(pkg).drain().unwrap();
    }

    // Batch exmpale
    let mut tree = Tree::batch(Etc::from(example)).unwrap();
    tree.load().unwrap();
    tree.redir(env::temp_dir()).unwrap();

    // Write example to file
    Etc::from(PathBuf::from("src/template.rs"))
        .write(
            [
                "/// Evlis APP Tempalte tree source",
                "(This file is auto-generated by `/build.rs`)\n",
                &format!(
                    "pub const APP_TEMPLATE: &str = r#\"{}\"#;\n",
                    &toml::to_string(&tree).unwrap()
                ),
            ]
            .join(""),
        )
        .unwrap();
}
