use camino::Utf8PathBuf;
use clap::{ArgEnum, Args, Parser, Subcommand};

/// Here is the app
/// peri build -- create a lock file from a spec (.lock)
/// peri publish -- upload a lock file (optional "app")
/// peri create -- create a spec file .toml from a list of packages 
/// peri login -- login to perihub
/// peri logout 
/// peri package -- create a tarball or docker container or sh file
#[derive(Debug, Parser)]
#[clap(name="peri", version)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts;

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Help message for Build.
    Build(BuildArgs),

    /// Help message for Publish.
    Publish(PublishArgs),

    /// Help message for Create.
    Create(CreateArgs),
   
    /// Help message for Login.
    Login(LoginArgs),

    /// Help message for Logout.
    Logout(LogoutArgs),

    /// Help message for Package.
    Package(PackageArgs),
}

#[derive(Debug, Args)]
struct BuildArgs {
    /// The path to the spec file
    specfile: Utf8PathBuf,
}

#[derive(Debug, Args)]
struct PublishArgs {
    /// The path to write to
    lockfile: Utf8PathBuf,
}

#[derive(Debug, Args)]
struct CreateArgs {
    /// The path to write to
    lockfile: Utf8PathBuf,
}

#[derive(Debug, Args)]
struct LoginArgs {
    /// The URL server to login to
    url: Utf8PathBuf,
}

#[derive(Debug, Args)]
struct LogoutArgs {
}

#[derive(Debug, Args)]
struct PackageArgs {
    /// The path to write to
    lockfile: Utf8PathBuf,
    // a list of other write args
}

#[derive(Debug, Args)]
struct GlobalOpts {
}

fn main() {
    println!("Hello, world!");
}
