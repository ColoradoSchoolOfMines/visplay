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
extern crate http;

use http::uri::Uri;
use failure::Error;
use serde::Deserialize;

mod uri_serde {
    use std::str::FromStr;
    use http::uri::Uri;
    use serde::{Serialize, Serializer, Deserialize, Deserializer};

    pub fn serialize<S>(uri: &Uri, ser: S) -> Result<S::Ok, S::Error> where S: Serializer {
        format!("{}", uri).serialize(ser)
    }

    pub fn deserialize<'de, D>(de: D) -> Result<Uri, D::Error> where D: Deserializer<'de> {
        use serde::de::Error as DeError;
        let string = String::deserialize(de)?;
        Uri::from_str(&string).map_err(DeError::custom)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Import {
    #[serde(with="uri_serde")]
    pub path: Uri,
    pub name: String,
    pub priority: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Added {
    Single {
        #[serde(with="uri_serde")]
        path: Uri,
        single: String,
    },
    Many {
        #[serde(with="uri_serde")]
        path: Uri,
        many: String,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Source {
    pub import: Vec<Import>,
    pub add: Vec<Added>,
}

pub fn de_path<D>(path: Uri, hint: Option<&str>) -> Result<D, Error>
    where D: for<'de> Deserialize<'de>
{
    let errmsg = || format!("can not load data at \"{}\"", path);

    match path.scheme() {
        None | Some("file") | Some("files") => {
            use std::ffi::OsStr;
            use std::fs::File;

            let path: &std::path::Path = path.path().as_ref();
            let file = File::open(path)?;
            let hint = hint.or(path.extension().and_then(OsStr::to_str));

            match hint {
                Some("json") => Ok(serde_json::from_reader(file)?),
                Some("yml") | Some("yaml") => Ok(serde_yaml::from_reader(file)?),
                Some("mp") | Some("msgpack") => Ok(rmp_serde::from_read(file)?),
                Some(form) => bail!("{}: unknown format \"{}\"", errmsg(), form),
                None => bail!("{}: cannot infer format", errmsg()),
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
    pub fn load(path: Uri, format_hint: Option<&str>) -> Result<Self, Error> {
        de_path(path, format_hint)
    }
}

#[test]
fn test_load_example_source() {
    use std::fs::File;
    use std::str::FromStr;

    let s: Source = serde_yaml::from_reader(File::open("../sources.yaml.example").unwrap()).unwrap();
    assert_eq!(s, Source {
        import: vec![
            Import { path: Uri::from_str("http://example.com/sources.yml").unwrap(), name: "example_com".to_owned(), priority: 0 },
            Import { path: Uri::from_str("http://acm.mines.edu/vis.yml").unwrap(), name: "acm".to_owned(), priority: 100 }
        ],
        add: vec! [
            Added::Many { path: Uri::from_str("/home/user/visplay/assets.yml").unwrap(), many: "asset".to_owned() },
            Added::Many { path: Uri::from_str("/home/user/visplay/playlists.yml").unwrap(), many: "playlist".to_owned() },
            Added::Single { path: Uri::from_str("/home/user/visplay/startup.yml").unwrap(), single: "playlist".to_owned() },
        ]
    });
}

pub struct Config {
    pub dbpath: String,
    pub roots: Vec<Source>,
}

pub struct Universe {

}

impl Universe {
    pub fn new(cfg: Config) -> Universe {
        unimplemented!()
    }
}
