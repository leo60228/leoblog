use anyhow::{ensure, Context, Result};
use serde::Deserialize;
use std::fs::{self, File};
use std::io::{prelude::*, stdin, BufReader, ErrorKind};
use std::path::PathBuf;
use toml_edit::{table, value, Document};

#[derive(Deserialize, Debug)]
pub struct Original {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct WebmentionResponse {
    pub original: Original,
}

fn add_frontmatter(target: &str, url: &str, path: PathBuf) -> Result<()> {
    let key = match target {
        "https://brid.gy/publish/mastodon" => "mastodon",
        x => {
            println!("unknown target: {:?}", x);
            return Ok(());
        }
    };
    println!("key: {}", key);

    let in_file = File::open(&path)?;
    let mut r = BufReader::new(in_file);
    let mut buf = String::new();

    r.read_line(&mut buf)?;
    if buf.trim() != "+++" {
        println!("no frontmatter");
        return Ok(());
    }

    buf.clear();
    while buf.trim().lines().last() != Some("+++") {
        r.read_line(&mut buf)?;
    }

    let frontmatter_str = buf
        .trim()
        .strip_suffix("+++")
        .context("couldn't strip trailing +++")?;

    let mut frontmatter: Document = frontmatter_str.parse()?;

    let extra = frontmatter
        .as_table_mut()
        .entry("extra")
        .or_insert(table())
        .as_table_mut()
        .context("Extra isn't a table!")?;

    *extra.entry(key) = value(url.to_string());

    let new_frontmatter = frontmatter.to_string_in_original_order();
    let mut new_file_str = format!("+++\n{}+++\n", new_frontmatter);
    r.read_to_string(&mut new_file_str)?;

    fs::write(&path, &new_file_str)?;

    Ok(())
}

fn main() -> Result<()> {
    let stdin = stdin();
    let mut file = stdin.lock();

    let mut buf = vec![];

    let mut source = "".to_string();
    let mut target = "".to_string();

    loop {
        buf.clear();
        file.read_until(b'=', &mut buf)?;
        if buf.is_empty() {
            break;
        }

        let key = std::str::from_utf8(&buf[..buf.len() - 1])?.trim();
        if key.is_empty() {
            break;
        }

        match key {
            "source" => {
                source.clear();
                file.read_line(&mut source)?;
                source = source.trim().to_string();
            }
            "target" => {
                target.clear();
                file.read_line(&mut target)?;
                target = target.trim().to_string();
            }
            "error" => {
                let deserializer = serde_json::Deserializer::from_reader(&mut file);
                let resp: WebmentionResponse = deserializer
                    .into_iter()
                    .next()
                    .context("Missing response!")??;
                let url = resp.original.url;
                let slug = source.split('/').nth(3).context("Couldn't extract slug!")?;
                let mut path = PathBuf::from("content/");
                path.push(&slug);
                path.set_extension("md");
                ensure!(path.is_file(), "Couldn't find file for {}!", url);
                println!("{} from {:?} made {}", target, path, url);
                add_frontmatter(&target, &url, path)?;
            }
            x => println!("ignoring {}", x),
        }

        match file.read_line(&mut String::new()) {
            Ok(_) => {}
            Err(x) if x.kind() == ErrorKind::UnexpectedEof => break,
            Err(x) => return Err(x.into()),
        }
    }

    Ok(())
}
