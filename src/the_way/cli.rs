//! `StructOpt` data
use std::path::PathBuf;

use structopt::clap::AppSettings;
use structopt::clap::Shell;
use structopt::StructOpt;

use crate::configuration::ConfigCommand;
use crate::the_way::filter::Filters;

#[derive(Debug, StructOpt)]
#[structopt(
name = "the-way",
rename_all = "kebab-case",
global_settings = & [AppSettings::DeriveDisplayOrder]
)]
/// Record, retrieve, search, and categorize code snippets
pub enum TheWayCLI {
    /// Add a new code snippet
    New,
    /// Add a new shell snippet
    Cmd {
        /// shell snippet code
        code: Option<String>,
    },
    /// Fuzzy search to find a snippet and copy, edit or delete it
    Search {
        #[structopt(flatten)]
        filters: Filters,
        // Print to stdout instead of copying (with Enter)
        #[structopt(long)]
        stdout: bool,
    },
    /// Sync snippets to a Gist
    ///
    /// Controlled by $THE_WAY_GITHUB_TOKEN env variable.
    /// Set this to an access token with the "gist" scope obtained from https://github.com/settings/tokens/new
    Sync,
    /// Lists (optionally filtered) snippets
    List {
        #[structopt(flatten)]
        filters: Filters,
    },
    /// Imports code snippets from JSON.
    ///
    /// Looks for description, language, and code fields.
    Import {
        /// filename, reads from stdin if not given
        #[structopt(parse(from_os_str))]
        file: Option<PathBuf>,

        #[structopt(long, short)]

        /// URL to a Gist, if provided will import snippets from given Gist
        ///
        /// Multiple files will be converted to separate snippets.
        /// Snippet description is created based on Gist description and file name with the format
        /// "<gist_description> - <gist_id> - <file_name>".
        /// Each snippet will be tagged with "gist" and its Gist ID.
        /// Works for both secret and public gists.
        gist_url: Option<String>,
    },
    /// Saves (optionally filtered) snippets to JSON.
    Export {
        /// filename, writes to stdout if not given
        #[structopt(parse(from_os_str))]
        file: Option<PathBuf>,
        #[structopt(flatten)]
        filters: Filters,
    },
    /// Clears all data
    Clear {
        /// Don't ask for confirmation
        #[structopt(long, short)]
        force: bool,
    },
    /// Generate shell completions
    Complete {
        #[structopt(possible_values = & Shell::variants())]
        shell: Shell,
    },
    /// Manage syntax highlighting themes
    Themes {
        #[structopt(subcommand)]
        cmd: ThemeCommand,
    },
    /// Manage the-way data locations.
    ///
    /// Controlled by $THE_WAY_CONFIG env variable,
    /// use this to have independent snippet sources for different projects.
    #[structopt(alias = "configure")]
    Config {
        #[structopt(subcommand)]
        cmd: ConfigCommand,
    },
    /// Change snippet
    Edit {
        /// Index of snippet to change
        index: usize,
    },
    /// Delete snippet
    #[structopt(alias = "delete")]
    Del {
        /// Index of snippet to delete
        index: usize,
        /// Don't ask for confirmation
        #[structopt(long, short)]
        force: bool,
    },
    /// Copy snippet to clipboard
    #[structopt(alias = "copy")]
    Cp {
        /// Index of snippet to copy
        index: usize,
        /// Print to stdout instead of copying
        #[structopt(long)]
        stdout: bool,
    },
    /// View snippet
    View {
        /// Index of snippet to show
        index: usize,
    },
}

#[derive(StructOpt, Debug)]
pub enum ThemeCommand {
    /// Set your preferred syntax highlighting theme
    Set { theme: Option<String> },
    /// Add a theme from a Sublime Text ".tmTheme" file.
    Add {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    /// Add highlight support for a language using a ".sublime-syntax" file.
    Language {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    /// Prints the current theme name
    Get,
}
