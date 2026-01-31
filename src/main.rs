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
    Program {
        operation : String,
        args      : Vec<String>
    },
    Project {
        operation : String,
        args      : Vec<String>
    }
    
}
fn main() {
    let cli = Cli::parse();
    let db_path = cli.database;
    println!("Database selected is {}", db_path);
    let db = database::KilnDatabase::new(&db_path);
    let mut db = db.unwrap();

    let command = cli.command;
    match command {
        Commands::Kiln{operation: operation, args: args} => 
            kiln(&mut db, &operation, args),
        Commands::Program {operation: op, args: args}    => 
            program(&mut db, &op, args ),        
        Commands::Project { operation: op, args: args} => 
            println!("project {} {:?}", op, args),
    };
    
}
// Process the kiln command:
//  kiln create name [description]    # Define a new kiln.
//  kiln list                         # List the names of all kilns.
//  kiln info kiln-name               # Describe the named kiln.
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
// Manipulate kiln programs:
// program create name kiln-name [description] # Define a new program on a kiln.
// program list kiln-name                      # Lists the names of program on a kiln.
// program info kiln-name program-name         # Describes a program on a kiln:
// program add-step kiln-name program-name ramp target dwell # Adds a step to a kiln program.
//       Note that 'ramp' can be AFAP for as fast as possible else deg/sec integer.
//       Note that target is integer degrees.
//       Note that dwell time is integer minutes.
fn program(db : &mut database::KilnDatabase, operation : &str, args : Vec<String>) {
    if operation == "create" {
        
    } else if operation == "list" {

    } else if operation == "info" {

    } else if operation == "add-step" {

    } else {
        eprintln!("Invalid 'program' subcommand: '{}'", operation);
    }
}