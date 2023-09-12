use std::fmt::Display;
use std::collections::HashSet;
use std::str::FromStr;
use anyhow::bail;
use enum_iterator::{all, Sequence};
use serde::Serialize;
use itertools::Itertools;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Sequence)]
#[serde(into = "&'static str")]
pub enum QuickwitService {
    ControlPlane,
    Indexer,
    Searcher,
    Janitor,
    Metastore,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Sequence)]
#[serde(into = "&'static str")] // 枚举可序列化为字符串, 但需要实现 Into
pub enum QuickwitServiceSet {
    ControlPlane,
    Indexer,
    Searcher,
    Janitor,
    Metastore,
}

#[allow(clippy::from_over_into)]
impl Into<&'static str> for QuickwitService {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

#[allow(clippy::from_over_into)]
impl Into<&'static str> for QuickwitServiceSet {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

impl QuickwitServiceSet {
    pub fn  as_str(&self) -> &'static str {
        match self {
            QuickwitServiceSet::ControlPlane => "control_plane",
            QuickwitServiceSet::Indexer => "indexer",
            QuickwitServiceSet::Searcher => "searcher",
            QuickwitServiceSet::Janitor => "janitor",
            QuickwitServiceSet::Metastore => "metastore",
        }
    }
}

impl QuickwitService {
    pub fn as_str(&self) -> &'static str {
        match self {
            QuickwitService::ControlPlane => "control_plane",
            QuickwitService::Indexer => "indexer",
            QuickwitService::Searcher => "searcher",
            QuickwitService::Janitor => "janitor",
            QuickwitService::Metastore => "metastore",
        }
    }

    pub fn supported_services() -> HashSet<QuickwitService> {
        all::<QuickwitService>().collect()
    }
}


impl Display for QuickwitService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for QuickwitService {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "control_plane" => Ok(QuickwitService::ControlPlane),
            "indexer" => Ok(QuickwitService::Indexer),
            "searcher" => Ok(QuickwitService::Searcher),
            "janitor" => Ok(QuickwitService::Janitor),
            "metastore" => Ok(QuickwitService::Metastore),
            _ => bail!(
                "Failed to parse service `{s}`. Supported services area: `{}`.", QuickwitService::supported_services().iter().join("`, `"))
        }
    }
}


fn main() {
    println!("Hello, world!");

    let service_name = QuickwitService::ControlPlane.as_str();
    println!("service name: {}", service_name);

    let indexer: &str = QuickwitService::Indexer.into();
    println!("indexer: {}", indexer);

    let hashset_indexer = QuickwitService::from_str("test");
    match hashset_indexer {
        Ok(qs) => {
            println!("hashset indexer: {:?}", qs);
        }
        Err(e) => {
            println!("error: {:?}", e.to_string());
        }
    }
}
