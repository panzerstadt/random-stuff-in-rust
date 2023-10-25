use std::{fs::DirEntry, path::Path};

trait Methods {
    fn has(&self, opt: Opt) -> bool;
}

#[derive(Debug)]
struct Args {
    raw: String,
}

impl From<&str> for Args {
    fn from(value: &str) -> Self {
        if !value.starts_with("-") {
            return Args {
                raw: String::default(),
            };
        }

        let args = value.get(1..).unwrap_or(""); // skip "-"
        return Args {
            raw: String::from(args),
        };
    }
}

enum Opt {
    All,
    AlmostAll,
    Reverse,
    Recursive,
}

impl Methods for Args {
    fn has(&self, opt: Opt) -> bool {
        match opt {
            Opt::All => self.raw.contains("a") || self.raw == "-all",
            Opt::AlmostAll => self.raw.contains("A") || self.raw == "-almost-all",
            Opt::Recursive => self.raw.contains("R") || self.raw == "-recursive",
            Opt::Reverse => self.raw.contains("r") || self.raw == "-reverse",
        }
    }
}

pub fn basic_ls_with_args(dir: &Path) -> Result<(), String> {
    let default_args = &String::from("");

    let args_vec = std::env::args().skip(1).collect::<Vec<_>>();
    let args_str = args_vec.first().unwrap_or(default_args).as_str();
    let args = Args::from(args_str);
    // println!("{:?} args", args);

    if dir.is_dir() {
        if args.has(Opt::All) {
            println!(".\n.."); // -____-"
        }
        let mut entries = dir
            .read_dir()
            .expect(format!("Error reading directory: {:?}", dir).as_str())
            .into_iter()
            .filter(|e| {
                // if -a
                if args.has(Opt::All) || args.has(Opt::AlmostAll) {
                    return true;
                }
                let e = e.as_ref().unwrap();
                let hidden_file = e
                    .path()
                    .file_stem()
                    .expect("Should be a file name")
                    .to_str()
                    .unwrap()
                    .starts_with(".");

                return !hidden_file;
            })
            .map(|entry| {
                let entry = entry.expect("Expect Directory Entry");
                let read_str = entry
                    .file_name()
                    .into_string()
                    .expect("Error parsing entry to string");

                println!("{}", read_str);

                return entry;
            })
            .collect::<Vec<DirEntry>>();

        entries.sort_by(|a, b| a.path().cmp(&b.path()));
        if args.has(Opt::Reverse) {
            entries.reverse();
        }

        entries.into_iter().for_each(|entry| {
            let path = entry.path();
            let file_name = path.to_str().expect("Error parsing entry to string");

            let meta = entry.metadata().unwrap();
            // -R
            if meta.is_dir() && args.has(Opt::Recursive) {
                println!("\n{}:", file_name);
                let _ = basic_ls_with_args(Path::new(&file_name));
            }

            return ();
        })
    }
    return Ok(());
}
