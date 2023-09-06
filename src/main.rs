use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short = 't', long, global = true, env = "ACCESS_TOKEN")]
    access_token: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    #[command(long_about = "Commands related to employees")]
    Employee {
        #[command(subcommand)]
        cmd: EmployeeCommand,
    },
    #[command(long_about = "Commands related to emails")]
    Email {
        #[command(subcommand)]
        cmd: EmailCommand,
    },
}

#[derive(Subcommand)]
enum EmployeeCommand {
    Get { id: u32 },
    Create { name: String },
}

#[derive(Subcommand)]
enum EmailCommand {
    List { id: u32 },
    Create { name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Employee { cmd } => match cmd {
            EmployeeCommand::Create { name } => {
                println!("Create the employee with the name: {}", name);
            }
            EmployeeCommand::Get { id } => {
                println!("Get the employee with the id: {}", id);
            }
        },
        Command::Email { cmd } => match cmd {
            EmailCommand::Create { name } => {
                println!("Create an email with the name: {}", name);
            }
            EmailCommand::List { id } => {
                println!("List the email with the id: {}", id);
            }
        },
    }

    match &cli.access_token {
        Some(value) => println!("Access token provided: {}", value),
        None => println!("No access token provided"),
    }
}
