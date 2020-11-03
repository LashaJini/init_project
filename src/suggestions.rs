// #[macro_use]
// use super::log;
// use super::env_logger;

// use super::log::info;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command};

type Callback = fn() -> Result<process::ExitStatus, io::Error>;

#[allow(non_snake_case)]
pub struct Project {
    AVAILABLE_PROJECTS: Vec<&'static str>,
    CALLBACKS: Vec<Callback>,
    num_of_projects: usize,
    key: &'static str,
}

// public
impl Project {
    pub fn new() -> Project {
        let num_of_projects = 4;
        let key = "";

        Project {
            AVAILABLE_PROJECTS: vec!["deno", "rust-wasm", "cargo-bin", "cargo-lib"],
            CALLBACKS: vec![
                Project::deno_handler,
                Project::rust_wasm_handler,
                Project::cargo_bin_handler,
                Project::cargo_lib_handler,
            ],
            num_of_projects,
            key,
        }
    }

    // TODO: logging
    pub fn display(self) -> Project {
        // env_logger::init();
        // info!("Uh");

        io::stdout().write_all(b" Choose one project:\n").unwrap();

        let mut i = 1;
        let _ = self
            .AVAILABLE_PROJECTS
            .iter()
            .map(|x| {
                println!(" {}: {}", i, x);
                i += 1;
                x
            })
            .collect::<Vec<_>>();
        println!();

        self
    }

    pub fn choose(mut self) -> Project {
        let key: &str = loop {
            io::stdout().write_all(b" >> ").unwrap();
            let _ = io::stdout().flush();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(val) if val >= 1 && val <= self.num_of_projects => {
                    break self.AVAILABLE_PROJECTS[val - 1];
                }
                _ => {
                    println!(
                        " [!!] Input should be numbers between {} and {}.",
                        1, self.num_of_projects
                    );
                    continue;
                }
            };
        };
        self.key = key;

        self
    }

    pub fn generate_project(self) {
        let functions: HashMap<&str, &Callback> = self
            .AVAILABLE_PROJECTS
            .into_iter()
            .zip(self.CALLBACKS.iter())
            .collect();

        let _ = functions.get(self.key).unwrap()();
    }
}

// private
impl Project {
    fn rust_wasm_handler() -> Result<process::ExitStatus, io::Error> {
        // cargo generate --git https://github.com/rustwasm/wasm-pack-template
        if let Ok(mut child) = Command::new("cargo")
            .arg("generate")
            .arg("--git")
            .arg("https://github.com/rustwasm/wasm-pack-template")
            .spawn()
        {
            child.wait()
        } else {
            eprintln!(" [!!!] Could not generate rust-wasm");
            process::exit(1);
        }
    }

    fn deno_handler() -> Result<process::ExitStatus, io::Error> {
        let project_name = &Project::read_project_name();

        let dirs = [
            "public/javascripts",
            "public/style",
            "tests",
            "routes",
            // format!("{}/public/javascripts", project_name),
            // format!("{}/public/style", project_name),
        ];

        let files = [
            "public/index.html",
            "public/javascripts/script.js",
            "public/style/style.css",
            "src/deps.ts",
            "src/mod.ts",
            "src/test_deps.ts",
            "DockerFile",
            "DrakeFile.ts",
            "lock.json",
        ];

        // create project root directory
        let mut cmd = Command::new("mkdir");
        cmd.arg("-p")
            .arg(project_name)
            .status()
            .expect(&format!("failed to create {}", project_name)[..]);

        // create sub directories
        let mut cmd = Command::new("mkdir");
        cmd.current_dir(fs::canonicalize(project_name).unwrap());
        cmd.arg("-p").args(&dirs).status()?;

        // create files
        let mut cmd = Command::new("touch");
        cmd.current_dir(fs::canonicalize(project_name).unwrap());
        cmd.args(&files).status()?;

        let path = Path::new(".");
        let path = path.join(project_name);

        let drakefile_ts_path = path.join("DrakeFile.ts");
        let data = Project::echo_drakefile_ts();
        fs::write(drakefile_ts_path, data).expect("Unable to write to DrakeFile.ts file");

        let deps_ts_path = path.join("deps.ts");
        let data = Project::echo_deps_ts();
        fs::write(deps_ts_path, data).expect("Unable to write to deps.ts file");

        let index_html_path = path.join("public/index.html");
        let data = Project::echo_index_html();
        fs::write(index_html_path, data).expect("Unable to write to public/index.html file");

        // this is bad; executing touch cmd again just to get te return type correctly
        cmd.status()
    }

    fn echo_drakefile_ts() -> String {
        "import { desc, task, sh, run } from \"./deps.ts\";

desc(\"start app\");
task(\"start\", [], async function () {
  // Add permissions
  await sh(
    \"deno run mod.ts\",
  );
});

run();"
            .to_string()
    }

    fn echo_deps_ts() -> String {
        "// Standard library dependencies
export * as log from \"https://deno.land/std/log/mod.ts\";

// Third party dependencies
export { desc, task, sh, run } from \"https://deno.land/x/drake/mod.ts\";
"
        .to_string()
    }

    fn echo_index_html() -> String {
        "<!DOCTYPE html>
<html lang=\"en\">
<head>
  <meta charset=\"UTF-8\">
  <title>Deno Project</title>
  <link rel=\"stylesheet\" href=\"style/style.css\">
</head>
<body>
  <h1>Deno Project</h1>
  <script src=\"javascripts/script.js\"></script>
</body>
</html>
"
        .to_string()
    }

    fn cargo_bin_handler() -> Result<process::ExitStatus, io::Error> {
        let project_name = &Project::read_project_name();

        let flag = "--bin";
        Project::cargo_handler(flag, project_name)
    }

    fn cargo_lib_handler() -> Result<process::ExitStatus, io::Error> {
        let project_name = &Project::read_project_name();

        let flag = "--lib";
        Project::cargo_handler(flag, project_name)
    }

    fn cargo_handler(flag: &str, project_name: &String) -> Result<process::ExitStatus, io::Error> {
        if let Ok(mut child) = Command::new("cargo")
            .args(&["new", flag, project_name])
            .spawn()
        {
            child.wait()
        } else {
            eprintln!(" [!!!] Could not generate cargo-bin");
            process::exit(1);
        }
    }

    fn read_project_name() -> String {
        // TODO: filter user input
        io::stdout().write_all(b" Project Name: ").unwrap();
        let _ = io::stdout().flush();

        let mut project_name = String::new();
        io::stdin().read_line(&mut project_name).unwrap();

        project_name.trim().to_string()
    }
}
