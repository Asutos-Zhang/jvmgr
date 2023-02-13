pub struct Config {
    names: Vec<String>,
    version_maps: Map<String, String>,
    versions: Vec<Version>,
}

struct Version {
    name: String,
    path: String,
}

impl Config {

    pub fn readAllFromFile(file:&str) -> Result<Vec<Version>> {
        let mut file = fs::File::create(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Vec::new())
    }

    pub fn readCurrent(file:&str) ->Result<Version> {
        Version {
            "".to_string(),
            "".to_string(),
        }
    }

    pub fn writeToFile(file:&str,alias:&str,jdk_path:&str) -> Result<(),Error> {
        let mut file = fs::File::create(file)?;
        
        Ok(())
    }
}
