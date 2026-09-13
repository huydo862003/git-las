pub const GITLAS_DIR: &str = ".gitlas";
pub const GITLAS_LOCAL_PATH: &str = ".gitlas/.local";
pub const CONFIG_FILE: &str = "config.toml";
pub const SECRETS_PATH: &str = ".gitlas/.local/secrets.toml";
pub const LOCAL_GITIGNORE: &str = r#"*
!.gitignore
"#;

// Global config lives in $HOME/.config/git-las/
pub const GLOBAL_CONFIG_SUBDIR: &str = ".config/git-las";
pub const GLOBAL_SECRETS_FILE: &str = "secrets.toml";

// File headers written at the top of generated TOML files
pub const CONFIG_HEADER: &str = r#"# git-las workspace config
#
# [meta]
# repo = "gitlas"           # name of the .gitlas/ meta-repo on remotes
#
# [remotes.<name>]          # declare a hosting provider (name = provider, e.g. github)
# url  = "https://github.com"
# user = "myorg"            # default user/org for all repos on this remote
#
# [repo.<name>]             # track a repo in this workspace
# [repo.<name>.primary]     # remote to pull from
# name = "github"
# [[repo.<name>.remotes]]   # additional push targets (defaults to all remotes)
# name = "gitlab"
#
# Run `git las remote add --help` and `git las repo --help` for details

"#;

pub const SECRETS_HEADER: &str = r#"# git-las secrets - DO NOT COMMIT
# Tokens for each remote, keyed by remote name
#
# [remotes.<name>]
# token = "ghp_..."

"#;
