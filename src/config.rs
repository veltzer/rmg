use crate::cli::Cli;

pub struct AppConfig {
    // Output control
    pub terse: bool,
    pub no_header: bool,
    pub no_output: bool,
    pub verbose: bool,
    pub print_not: bool,

    // Main
    pub no_sort: bool,
    pub glob: String,
    pub no_glob: bool,
    pub folders: Vec<String>,
    pub no_stop: bool,
    pub no_print_no_projects: bool,
    pub jobs: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            terse: false,
            no_header: false,
            no_output: false,
            verbose: false,
            print_not: false,
            no_sort: false,
            glob: "*/*".to_string(),
            no_glob: false,
            folders: Vec::new(),
            no_stop: false,
            no_print_no_projects: false,
            jobs: 1,
        }
    }
}

impl From<&Cli> for AppConfig {
    fn from(cli: &Cli) -> Self {
        Self {
            terse: cli.terse,
            no_header: cli.no_header,
            no_output: cli.no_output,
            verbose: cli.verbose,
            print_not: cli.print_not,
            no_sort: cli.no_sort,
            glob: cli.glob.clone(),
            no_glob: cli.no_glob,
            folders: cli.folders.clone(),
            no_stop: cli.no_stop,
            no_print_no_projects: cli.no_print_no_projects,
            jobs: cli.jobs,
        }
    }
}
