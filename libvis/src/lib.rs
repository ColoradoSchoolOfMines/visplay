#[macro_use]
extern crate diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
extern crate rmp_serde;
#[macro_use]
extern crate failure;
extern crate mime;
extern crate mime_guess;
extern crate hyper;
extern crate easy_uri as uri;

use std::fmt;
use std::str::FromStr;

use mime::Mime;
use uri::Uri;
use failure::Error;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::collections::{VecDeque, HashMap};
use std::path::{Path, PathBuf};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct WildUri {
    parts: Vec<String>,
}

impl WildUri {
    pub fn new(uri: &str) -> WildUri {
        WildUri {
           parts: uri.split("*").map(ToOwned::to_owned).collect()
       }
    }

    
    pub fn render(&self, fill: &str) -> Result<Uri, Error> {
        Ok(Uri::from_str(&self.render_str(fill))?)
    }

    pub fn render_str(&self, fill: &str) -> String {
        if let Some(first) = self.parts.get(0) {
            let mut out = first.clone();
            for part in &self.parts[1..] {
                out += fill;
                out += part;
            }
            out
        } else {
            String::new()
        }
    }

    pub fn wild_str(&self) -> String {
        self.render_str("*")
    }
}

impl fmt::Debug for WildUri {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.wild_str())
    }
}

impl<'de> Deserialize<'de> for WildUri {
    fn deserialize<D>(de: D) -> Result<WildUri, D::Error> where D: Deserializer<'de> {
       Ok(WildUri::new(&String::deserialize(de)?))
    }
}

impl Serialize for WildUri {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error> where S: Serializer {
       self.wild_str().serialize(ser)
    }
}

impl FromStr for WildUri {
    type Err = (); // TODO: bang type
    fn from_str(s: &str) -> Result<WildUri, ()> {
        Ok(WildUri::new(s))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Import {
    pub path: Uri,
    pub name: String,
    #[serde(default)]
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Added {
    Single {
        path: Uri,
        single: String,
    },
    Many {
        path: Uri,
        many: String,
    },
    Wild {
        path: WildUri,
        wild: String,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Source {
    #[serde(default)]
    pub import: Vec<Import>,
    #[serde(default)]
    pub add: Vec<Added>,
}

pub fn de_path<D>(path: &Uri, hint: Option<Mime>) -> Result<D, Error>
    where D: for<'de> Deserialize<'de>
{
    let errmsg = || format!("can not load data at \"{}\"", path);

    match path.scheme.as_ref().map(String::as_str) {
        None | Some("file") | Some("files") => {
            use std::fs::File;

            let path = &path.path;

            let file = File::open(&path)?;
            let mime = match hint.or(mime_guess::guess_mime_type_opt(path)) {
                Some(m) => m,
                None => bail!("{}: cannot infer format", errmsg()),
            };

            let mut subtype = mime.subtype().as_str();
            if subtype.starts_with("x-") {
                subtype = &subtype[2..];
            }

            match subtype {
                "json" => Ok(serde_json::from_reader(file)?),
                "yaml" => Ok(serde_yaml::from_reader(file)?),
                "msgpack" => Ok(rmp_serde::from_read(file)?),
                form => bail!("{}: unknown format \"{}\"", errmsg(), form),
            }
        },
        Some("http") | Some("https") => {
            unimplemented!()
        },
        Some(scheme) => {
            bail!("{}: unknown scheme \"{}\"", errmsg(), scheme)
        },
    }
}

impl Source {
    pub fn open_uri(path: &Uri, format_hint: Option<Mime>) -> Result<Self, Error> {
        de_path(path, format_hint)
    }

    pub fn open<S>(path: S, format_hint: Option<Mime>) -> Result<Self, Error>
        where S: AsRef<str>
    {
        Source::open_uri(&Uri::from_str(path.as_ref())?, format_hint)
    }
}

#[test]
fn test_load_example_source() {
    let s = Source::open("../example.sources.yml", None).unwrap();
    assert_eq!(s, Source {
        import: vec![
            Import { path: Uri::from_str("http://example.com/sources.yml").unwrap(), name: "example_com".to_owned(), priority: 0 },
            Import { path: Uri::from_str("http://acm.mines.edu/vis.yml").unwrap(), name: "acm".to_owned(), priority: 100 }
        ],
        add: vec![
            Added::Many { path: Uri::from_str("/home/user/visplay/assets.yml").unwrap(), many: "asset".to_owned() },
            Added::Many { path: Uri::from_str("/home/user/visplay/playlists.yml").unwrap(), many: "playlist".to_owned() },
            Added::Single { path: Uri::from_str("/home/user/visplay/startup.yml").unwrap(), single: "playlist".to_owned() },
        ]
    });
}

#[test]
fn test_load_youtube_source() {
    let s = Source::open("../youtube.sources.yml", None).unwrap();
    assert_eq!(s, Source {
        import: vec![],
        add: vec![
            Added::Wild { path: WildUri {
                parts: vec!["data:text/yaml,-%20https%3A%2F%2Fyoutu.be%2F".to_owned(), "".to_owned()]
            }, wild: "asset".to_owned() },
        ]
    });
}

pub struct Config {
    pub dbpath: String,
    pub root: Source,
}

pub struct Universe {
    pub sources: HashMap<String, Source>,
}

impl Universe {
    pub fn new(cfg: Config) -> Universe {
        let mut u = Universe {
            sources: HashMap::new(),
        };
        u.sources.insert(":".to_owned(), cfg.root);
        u.expand();
        u
    }

    fn expand(&mut self) {
        fn add_to_queue(
            name: &str,
            queue: &mut VecDeque<(String, PathBuf, Import)>,
            source: &Source)
        {
            queue.extend(source.import.iter().cloned().map(|i| (
                name.to_owned(),
                i.path.path.parent().unwrap_or("/".as_ref()).to_owned(),
                i,
            )));
        }

        let mut queue = VecDeque::new();
        for (name, source) in &self.sources {
            add_to_queue(&name, &mut queue, &source);
        }

        while let Some((name, local, import)) = queue.pop_front() {
            match Source::open_uri(&import.path, None) {
                Ok(source) => {
                    let name = name.clone() + &import.name;
                    add_to_queue(&name, &mut queue, &source);
                    self.sources.insert(name, source);
                },
                Err(e) => eprintln!("{}", e),
            }
            
        }
    }
}
