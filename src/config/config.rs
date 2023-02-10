pub struct Config {
    names: Vec<String>,
    version_maps: Map<String, String>,
    versions: Vec<Version>,
}

struct Version {
    name: String,
    path: String,
}

impl Config {}
