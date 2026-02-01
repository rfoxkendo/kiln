
use kiln::*;
use clap::{Parser, Subcommand};
use std::process::exit;
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
        if args.len() < 2  {
            eprintln!("program create needs at least a program and kiln name");
            exit(-1);
        }
        let pgm_name = args[0].clone();
        let kiln_name = args[1].clone();
        let mut description  = String::from ("");  
        if args.len() == 3 {
            description = args[2].clone();
        }
        if args.len() > 3 {
            eprintln!("program create needs at most a program, kiln-name and description");
            exit(-1);
        }
        if let Err(e) = db.add_kiln_program(&pgm_name, &kiln_name, &description) {
            eprintln!("Failed to add program {} to kiln{} ({}) : {}", pgm_name, kiln_name, description, e);
            exit(-1);
        }
        return;
    } else if operation == "list" {
        if args.len() != 1 {
            eprintln!("program list requires just a kiln name");
            exit(-1);
        }
        let kiln = args[0].clone();
        match db.list_kiln_programs(&kiln) {
            Ok(list) => {
                println!("Prorams defined for kiln {}", kiln);
                for pgm in list {
                    println!("  {}", pgm);
                }
                return;
            },
            Err(e) => {
                eprintln!("Unable to list programs for kiln {}", kiln);
                exit(-1);
            },
        };

    } else if operation == "info" {
        if args.len() != 2 {
            eprintln!("program info requirews a kiln name and a program name");
        }
        let kiln = args[0].clone();
        let pgm = args[1].clone();
        match db.get_kiln_program(&kiln, &pgm) {
            Ok(info) => { 
                match info {
                    Some(p) => print_program(&pgm, &p),
                    None => eprintln!("No Such program in that kiln"),
                }
                return;
            },
            Err(e) => {
                eprintln!(
                    "Could not get information about program {} in kiln {} : {}",
                    pgm, kiln, e
                );
                exit(-1);
            }
        }

    } else if operation == "add-step" {

    } else {
        eprintln!("Invalid 'program' subcommand: '{}'", operation);
    }
}

// Print the details of a kiln program:

fn print_program(name : &str, pgm : &database::KilnProgram) {
    println!("Kiln: {} ({})", pgm.kiln().name(), pgm.kiln().description());
    println!("Program {} ({})", pgm.sequence().name(), pgm.sequence().description());
    let steps = pgm.steps();
    if steps.len() > 0 {
        println!("Firing steps:");
        for step in steps  {
            println!(
                "Ramp at {} deg/sec until {} deg hold for {} minutes",
                step.ramp_rate(), step.target_temp(), step.dwell_time()
            );
        }
    } else {
        println!("No steps defined yet.");
    }

}