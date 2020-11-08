//! Suggested Projects Module
//!
//! This module contains main functions for handling project creation.

// #[macro_use]
// use super::log;
// use super::env_logger;

// use super::log::info;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command};

/// Type for function callbacks
///
/// This type is used as value for `HashMap`. Functions that return this type are
/// project generators, which are *_handler functions.
type Callback = fn(project: &mut Project) -> Result<process::ExitStatus, io::Error>;

/// Contains all the necessary handler information
#[allow(non_snake_case)]
pub struct Project {
    AVAILABLE_PROJECTS: Vec<&'static str>,
    num_of_projects: usize,
    functions: HashMap<&'static str, Callback>,
    key: &'static str,
    project_name: String,
}

// public
impl Project {
    /// Creates the new instance of `Project` structure
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() {
    /// let project = Project::new();
    /// # }
    /// ```
    #[allow(non_snake_case)]
    pub fn new() -> Project {
        let num_of_projects = 5;
        let key = "";
        let project_name = String::from("");

        let AVAILABLE_PROJECTS: Vec<&str> = vec![
            "deno",
            "rust-wasm",
            "cargo-bin",
            "cargo-lib",
            "create-react-app",
        ];
        let CALLBACKS: Vec<Callback> = vec![
            Project::deno_handler,
            Project::rust_wasm_handler,
            Project::cargo_bin_handler,
            Project::cargo_lib_handler,
            Project::create_react_app_handler,
        ];

        let functions: HashMap<&str, Callback> = AVAILABLE_PROJECTS
            .into_iter()
            .zip(CALLBACKS.into_iter())
            .collect();

        Project {
            AVAILABLE_PROJECTS: vec![
                "deno",
                "rust-wasm",
                "cargo-bin",
                "cargo-lib",
                "create-react-app",
            ], // O_O
            num_of_projects,
            functions,
            key,
            project_name,
        }
    }

    /// Displays the projects to choose from
    ///
    /// Returns the instance of `Project`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() {
    /// let project = Project::new();
    /// project.display();
    /// # }
    /// ```
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

    /// Prompts the user to choose from the available projects
    ///
    /// Sets key to which project the user has chosen. Returns the instance of the `Project`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() {
    /// let project = Project::new();
    /// project.display().choose();
    /// # }
    /// ```
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

    /// Generate project
    ///
    /// Generates user chosen project.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() {
    /// let project = Project::new();
    /// project.display().choose().generate_project();
    /// # }
    /// ```
    pub fn generate_project(&mut self) {
        let _ = self.functions.get(self.key).unwrap()(self);
        Project::init_git_inside(self.get_project_name());
    }
}

// private
impl Project {
    fn set_project_name(&mut self, project_name: String) {
        self.project_name = project_name;
    }

    fn get_project_name(&self) -> String {
        let result = &self.project_name;
        result.to_string()
    }

    fn init_git_inside(project_name: String) {
        let mut cmd = Command::new("git");
        cmd.current_dir(fs::canonicalize(&project_name).unwrap());
        if let Ok(mut child) = cmd.arg("init").spawn() {
            // is dis good? :/
            match child.wait() {
                _ => (),
            }
        } else {
            eprintln!(" [!!!] Could not initialize Git inside {}", &project_name);
            eprintln!(" [!!] Exiting...");
            process::exit(1);
        }
    }

    fn rust_wasm_handler(_project: &mut Project) -> Result<process::ExitStatus, io::Error> {
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

    fn create_react_app_handler(project: &mut Project) -> Result<process::ExitStatus, io::Error> {
        let project_name = &Project::read_project_name();
        project.set_project_name(project_name.to_string());

        if let Ok(mut child) = Command::new("create-react-app")
            .arg(project.get_project_name())
            .spawn()
        {
            child.wait()
        } else {
            eprintln!(" [!!!] Could not create-react-app");
            process::exit(1);
        }
    }

    fn deno_handler(project: &mut Project) -> Result<process::ExitStatus, io::Error> {
        let project_name = &Project::read_project_name();
        project.set_project_name(project_name.to_string());

        let dirs = [
            "public/javascripts",
            "public/style",
            "public/res/images",
            "src/routes",
            "tests",
        ];

        let files = [
            "public/index.html",
            "public/javascripts/script.js",
            "public/style/style.css",
            "src/deps.ts",
            "src/mod.ts",
            "src/test_deps.ts",
            ".env",
            ".gitignore",
            "Dockerfile",
            "docker-compose.yml",
            "DrakeFile.ts",
            "lock.json",
            "README.md",
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

        let deps_ts_path = path.join("src").join("deps.ts");
        let data = Project::echo_deps_ts();
        fs::write(deps_ts_path, data).expect("Unable to write to deps.ts file");

        let index_html_path = path.join("public/index.html");
        let data = Project::echo_index_html();
        fs::write(index_html_path, data).expect("Unable to write to public/index.html file");

        let dot_env_path = path.join(".env");
        let data = Project::echo_dot_env();
        fs::write(dot_env_path, data).expect("Unable to write to .env file");

        let docker_compose_yml = path.join("docker-compose.yml");
        let data = Project::echo_docker_compose_yml(project_name.to_string());
        fs::write(docker_compose_yml, data).expect("Unable to write to docker-compose.yml file");

        // this is bad; executing touch cmd again just to get te return type correctly
        cmd.status()
    }

    fn echo_docker_compose_yml(project_name: String) -> String {
        format!(
            "version: '3'
services:
  api:
    container_name: {project_name}
    # image: hayd/deno:alpine-1.5.0
    environment:
      - SHELL=/bin/sh
    command: run --allow-all DrakeFile.ts start
    env_file: .env
    volumes:
      - .:/app
    working_dir: /app
    ports:
      # - \"8000:8000\"
     - \"${{PORT}}:${{PORT}}\"
    restart: always",
            project_name = project_name
        )
        .to_string()
    }

    fn echo_dot_env() -> String {
        "PORT=8000".to_string()
    }

    fn echo_drakefile_ts() -> String {
        "import { desc, task, sh, run } from \"./src/deps.ts\";

desc(\"start app\");
task(\"start\", [], async function () {
  // Add additional permissions
  await sh(
    \"deno run src/mod.ts\",
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
export { config } from \"https://deno.land/x/dotenv/mod.ts\";
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

    fn cargo_bin_handler(project: &mut Project) -> Result<process::ExitStatus, io::Error> {
        let project_name = &Project::read_project_name();
        project.set_project_name(project_name.to_string());

        let flag = "--bin";
        Project::cargo_handler(flag, project_name)
    }

    fn cargo_lib_handler(project: &mut Project) -> Result<process::ExitStatus, io::Error> {
        let project_name = &Project::read_project_name();
        project.set_project_name(project_name.to_string());

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
