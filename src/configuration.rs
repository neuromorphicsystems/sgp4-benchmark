pub fn root() -> std::path::PathBuf {
    std::fs::canonicalize(std::path::Path::new(file!()))
        .unwrap()
        .ancestors()
        .nth(2)
        .unwrap()
        .to_owned()
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BuildCommand {
    pub current_dir: String,
    pub command: String,
    pub args: Vec<String>,
}

impl BuildCommand {
    pub fn spawn(&self, root: &std::path::Path) -> std::io::Result<std::process::Child> {
        std::process::Command::new(&self.command)
            .args(&self.args)
            .current_dir(root.join(&self.current_dir))
            .spawn()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Executable {
    pub id: String,
    pub path: String,
}

impl Executable {
    pub fn run(&self, output: &str) -> std::io::Result<u64> {
        let root = root();
        let result = std::process::Command::new(root.join(&self.path))
            .args(&[root.join("omms.json").to_str().unwrap(), output])
            .output()?;
        Ok(std::str::from_utf8(&result.stdout)
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub build_commands: Vec<BuildCommand>,
    pub executables: Vec<Executable>,
}

impl Configuration {
    pub fn parse() -> Configuration {
        serde_json::from_str(include_str!("../configuration.json")).unwrap()
    }

    pub fn build(&self) -> std::io::Result<()> {
        let root = root();
        for build_command in &self.build_commands {
            let name = format!(
                "{} {} (directory {})",
                build_command.command,
                build_command.args.join(" "),
                root.join(&build_command.current_dir).to_str().unwrap()
            );
            println!("{}", name);
            let mut child = build_command.spawn(&root)?;
            if !child.wait()?.success() {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, name));
            }
        }
        Ok(())
    }
}
