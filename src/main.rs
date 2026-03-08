
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
            project(&mut db, &op, args),
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
        if let Err(e) = db.add_kiln_program(&kiln_name, &pgm_name, &description) {
            eprintln!("Failed to add program {} to kiln {} ({}) : {}", pgm_name, kiln_name, description, e);
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
                eprintln!("Unable to list programs for kiln {} : {}", kiln, e);
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
        add_program_step(db, args);              // Complex enough to deserve its own function.
    } else {
        eprintln!("Invalid 'program' subcommand: '{}'", operation);
    }
}

// Add a step to a kiln program

fn add_program_step(db : &mut database::KilnDatabase, args : Vec<String>) {
    // We need a kiln, program, ramp-rate, target and dwell.
    // The ramp-rate can be either "AFAP" or an integer degreees/hr.

    if args.len() != 5 {
        eprintln!("program add-step needs a kiln, a program-name, ramp-rate, target-temp and hold-time");
        exit(-1);
    }
    let kiln = args[0].clone();
    let program = args[1].clone();

    // Figure out the ramp rate:

    let ramp_rate = if args[2] == "AFAP" {
        database::RampRate::AFAP
    } else  {
        if let Ok(rate) = args[2].parse::<u32>() {
            database::RampRate::DegPerHr(rate)
        } else {
            eprintln!("Ramp rate must be either an integer or 'AFAP' not {}", args[2]);
            exit(-1);
        }
    };
    let target = match args[3].parse::<u32>() {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "Failed to convert ramp target {} to an unsigned value {}", 
                args[3], e
            );
            exit(-1);
        },
    };

    let dwell = match args[4].parse::<u32>() {
        Ok(d) => d,
        Err(e) => {
            eprintln!(
                "Unable to convert dwell time {} to an unsigned integer valueu {}",
                args[4], e
            );
            exit(-1);
        },
    };

    // Get the current definition (if we can) then add the step:

    let program_info = db.get_kiln_program(&kiln, &program);
    if let Err(e) = program_info {
        eprintln!("Unable to fetch program {} on kiln {}: {}", kiln, program, e);
        exit(-1);
    }
    let program_info = program_info.unwrap();
    if let None = program_info {
        eprintln!("No such program {} on kiln {}", kiln, program);
    }
    let mut program_info = program_info.unwrap();

    let new_step = database::FiringStep::new(0,0, ramp_rate, target, dwell);
    program_info.add_step(&new_step);

    if let Err(e) = db.update_kiln_program(&program_info) {
        eprintln!(
            "Could not add a step to the program {} in kiln {}: {}",
            program, kiln, e
        );
        exit(-1);
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
                "Ramp at {} deg/hr until {} deg hold for {} minutes",
                step.ramp_rate(), step.target_temp(), step.dwell_time()
            );
        }
    } else {
        println!("No steps defined yet.");
    }

}

// Handle the project subcommand:
// subcommands:
//   project create name [description]
//   project list
//   project info name
//   project add-firing project kiln program
//   project add-image project image-file [caption]
//  
fn project(db: &mut database::KilnDatabase, op : &str, args: Vec<String>) {

    if op == "create" {
        // Can be one or two parameters must be one.

        if args.len() < 1 {
            eprintln!("project create needs a project name");
            exit(-1);
        }
        if args.len() > 2 {
            eprintln!("project create too many command parameters.")
        }
        let project = args[0].clone();
        let desc    = if args.len() == 2 {
            args[1].clone()
        } else {
            String::from ("")
        };

        if let Err(e) = db.add_project(&project, &desc) {
            eprintln!(
                "Error adding project {} ({}) : {}",
                project, desc, e
            );
            exit(-1);
        }
        return;
    } else if op == "list" {
        if args.len() != 0 {
            eprintln!("the project list operation does not expect additional command parameters");
            exit(-1);
        }
        match db.list_projects() {
            Ok(l) => {
                println!("Project names:");
                for n in l {
                    println!(" {}", n);
                }
            },
            Err(e) => {
                eprintln!("Unable to compile project list: {}", e);
                exit(-1);
            }
        }
    } else if op == "info" {
        if args.len() != 1 {
            eprintln!("project info needs a project and only a project.");
            exit(-1);
        }
        let project = args[0].clone();
        let project_info = db.get_project(&project).unwrap();
        describe_project(&project, project_info);
    } else if op == "add-firing" {
        if args.len() < 3 || args.len() > 4  {
            eprintln!("add-firing requires a project a kiln and a program in that kiln and an optional comment.");
            exit(-1);
        }
        let project = args[0].clone();
        let kiln = args[1].clone();
        let program = args[2].clone();

        let comment = if args.len() == 4 {
            args[3].clone()
        } else {
            String::new()
        };
        // We need the project and the kiln program:

        let project_info = db.get_project(&project).unwrap().unwrap();
        let updated_project = db.add_project_firing(&project_info, &kiln, &program, &comment).unwrap();
        println!("Updated Project: ");
        describe_project(&project, Some(updated_project));
        
    } else {
        eprintln!("Unsupported or illegal operation: {}", op)
    }

}
// Describe a kiln project:

fn describe_project(name : &str, info : Option<database::KilnProject>) {
    if info.is_none() {
        eprintln!("There is no project named {}", name);
    } else {
        let info = info.unwrap();
        println!("Name: {} Description {}", info.project().name(), info.project().description());
        println!("Firings:");
        for (i, firing) in info.firings().iter().enumerate() {
            println!(
                "Firing in {}: {} - {}", 
                firing.kiln().name(), firing.kiln().description(), info.firing_comments()[i]
            );
            println!("  Steps for {} {}:", firing.sequence().name(), firing.sequence().description());
            for step in firing.steps().iter() {
                println!(
                    "   Ramp to {} degrees rate: {} Dwell: {} minutes", 
                    step.target_temp(), step.ramp_rate(), step.dwell_time()
                );
            }
        }
        println!("There are {} pictures attached to this project", info.num_images());
    }
}