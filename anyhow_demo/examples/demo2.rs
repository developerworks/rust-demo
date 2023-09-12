use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct ImportantThing {
    path: PathBuf,
}

impl ImportantThing {
    pub fn detach(&mut self) -> Result<()> {
        Err(anyhow!("detach faield"))
    }
}

pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
    it.detach()
        .context("Failed to detach the important thing")?;

    let path = &it.path;
    let content =
        fs::read(path).with_context(|| format!("Failed to read instrs from {}", path.display()))?;

    Ok(content)
}

fn main() {
    let mut it = ImportantThing {
        path: PathBuf::new(),
    };
    match do_it(it) {
        Ok(_) => (),
        Err(ref err) => {
            for cause in err.chain() {
                println!("{}", cause);
            }
        }
    }
}