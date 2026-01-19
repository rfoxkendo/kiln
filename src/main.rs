use std::f32::consts::E;

use kiln::*;
use clap::{Parser, Subcommand};

// Define the commands?

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(short,long)]
    database : String,
    #[command(subcommand)]
    command : Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Kiln {
        operation : String,
        args      : Vec<String>
    },
    Program {},
    Project {}
    
}
fn main() {
    let cli = Cli::parse();
    let db_path = cli.database;
    println!("Database selected is {}", db_path);
    let db = database::KilnDatabase::new(&db_path);
    let mut db = db.unwrap();

    let command = cli.command;
    match command {
        Commands::Kiln{operation: operation, args: args} => kiln(&mut db, &operation, args),
        Program    => println!("program"),
        Project => println!("project")
    };
    
}

fn kiln(db : &mut database::KilnDatabase, operation : &str, kiln_info : Vec<String>) {
    if operation == "list" {
        let kiln_list = db.list_kilns().unwrap();
        for kiln in kiln_list {
            println!("{}", kiln);
        }
    } else if operation == "create" {
        let mut description = String::from("");           // Allow no description.
        if kiln_info.len() == 0 || kiln_info.len() > 2 {
            eprintln!("Need a kiln name and at most a kiln name and description");
        } else {
            let name = kiln_info[0].clone();
            if kiln_info.len() == 2 {
                description = kiln_info[1].clone();
            }
            db.add_kiln(&name, &description).unwrap();
        }
    } else if operation == "info" {
        if kiln_info.len() == 1 {
            let info = db.get_kiln(&kiln_info[0]);
            match info.unwrap() {
                Some(info) => {
                    println!("Name       : {}", info.name());
                    println!("Description: {}", info.description());
                },
                None => eprintln!("No kiln named {}", kiln_info[0]),
            };
            
        } else {
            eprintln!("Need a kiln name for info");
        }

    } else {
        eprintln!("Invalid kiln subcommand");
    }
}