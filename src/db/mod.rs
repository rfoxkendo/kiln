//!  Provides sqlite3 support for saving/retrieving kiln programs
//! and kiln runs.
//! The tables that we define are
//! 
//! ### Programs - Root table for kiln firing programs
//! 
//! | Field | Type   | Description                                | 
//! |-------|--------|--------------------------------------------|
//! | id    | INTEGER | Primary key  |
//! | name  | TEXT    | Name of the program should be unique    |
//! | description | TEXT | What the program does.               |
//! 
//! ### Steps  - steps in programs.
//! | Field    | Type   | Description                           |
//! |----------|--------|---------------------------------------|
//! | id       | INTEGER | Primary key |
//! | program_id | INTEGER | Foreign key to programs program using this step |
//! | step_no  |  INTEGER | Step number - used to order the steps. |
//! | target   | REAL    | Target temparature |
//! | ramp_rate | REAL   | Ramp rate in deg/hr -1.0 means AFAP  |
//! | hold_time | INTEGER | Minutes to hold at that temperaturl |
//! 
//! ### Projects - Describes runs of programs.
//! 
//! | Field     | Type     | Description                     |
//! |-----------|----------|---------------------------------|
//! | id        | INTEGER  | Primary key                     |
//! | name      | TEXT     | Name of the project - should be unique |
//! | description | TEXT   | Description of the project      | 
//! | program_id | INTEGER | FK into the Programs table - which kiln program was used to fire. |
//! 
//! #### Images - Image files of program runs. 
//! 
//! | Field       |  Type    |  Description                  |
//! |-------------|----------|-------------------------------|
//! | id          | INTEGER  | Primary key                   |
//! | project_id  | INTGEGER | FK to Projects table which project this image belongs to |
//! | path        | TEXT     | Path to the image file.       |
//! 
//! 
//! 
use rusqlite::{self, AndThenRows};
use crate::lib::programs;

/// This stucture represents a database - it is used
/// to fetch and store data into a database. 
pub struct Database {
    pub connection : rusqlite::Connection
}

/// The implementation of the database.  Note that 
/// successful connection to a database file implies the creation (if needed)
/// of the tables.
impl Database {
     // Create the programs table.
     //
    fn create_programs(connection : &rusqlite::Connection) 
        -> Result<(), rusqlite::Error> {
        connection.execute(" 
            CREATE TABLE Programs IF NOT EXISTS (
                id  INTEGER PRIMARY KEY AUTO INCREMENT,
                name  TEXT,
                description  TEXT
            )
        "  , [])?;
        Ok(())
    }
    // Create the Steps table

    fn create_steps(connection : &rusqlite::Connection) 
        ->  Result<(), rusqlite::Error> {
        connection.execute("
            CREATE TABLE Steps IF NOT EXISTS (
                id          INTEGER PRIMARY KEY AUTO INCREMENT,
                program_id  INTEGER, -- FK to Programs
                step_no     INTEGER,
                target      REAL,
                ramp_rate   REAL,
                hold_time   INTEGER
            )
        ", [])?;

        Ok(())
    }

    // Create the projects table.

    fn create_projects(connection : &rusqlite::Connection) 
        ->  Result<(), rusqlite::Error> {
        connection.execute("
            CREATE TABLE Projects IF NOT EXISTS (
                id           INTEGER PRIMARY KEY AUTO INCREMENT,
                name         TEXT,
                description  TEXT,
                program_id   INTEGER -- FK to Programs table.
            )
        ", [])?;

        Ok(())
    }
    // Create the images table

    fn create_images(connection : &rusqlite::Connection) 
        ->  Result<(), rusqlite::Error> { 
        connection.execute("
            CREATE TABLE Images IF NOT EXISTS (
                id          INTEGER PRIMARY KEY AUTO INCREMENT,
                project_id  INTEGER -- FD to projects table.
                path        TEXT
            )
        ", [])?;

        Ok(())
    }
    // Do the database open:

    fn open(filename : &str) -> Result<rusqlite::Connection, rusqlite::Error> {
        let connection =rusqlite::Connection::open(filename)?;

        // Create db schema if needed.

        Self::create_programs(&connection)?;
        Self::create_steps(&connection)?;
        Self::create_projects(&connection)?;
        Self::create_images(&connection)?;

        Ok(connection)

    }
    /// Open a database, on success, the databae struct is returned,
    /// if not the rusqlite error message is returned instead.
    /// 
    ///    filename is the name of a file that is or will be the database file.
    pub fn new(filename : &str) -> Result<Database, rusqlite::Error> {
        match Self::open(filename) {
            Ok(connection) => Ok(Database {connection : connection}),
            Err(e) => Err(e)
        }
    }
}

///
/// A Result row is a generic type that contains both the
/// primary key and the underlying struct:
///
#[derive(Debug, Clone)]
pub struct Row <T>
where T : Clone,
{
    id : u32,
    row : T
}

impl <T>  Row<T>
where T: Clone,
{
    pub fn new  (id : u32, row : T) -> Row<T> {
        Row {id : id, row: row.clone()}
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn Contents(&self) -> T {
        self.row.clone()
    }

}

// Specific types that we'll use:

type Step = Row<programs::Step>;

#[derive(Debug, Clone)]
pub struct Program {
    id : u32,
    name: String,
    description : String,
    program : Vec<Step>
}

impl Program {
    // Just make from scratch.
    //
    pub fn new(id : u32, name : &str, description: &str, steps : &Vec<Step>) -> Program {
        Program {
            id : id, 
            name : String::from(name),
            description: String::from(description),
            program: steps.clone()
        }
    }
    /// Strip of the id parts to give a program::Program.
    pub fn toProgram(&self) -> programs::Program {
        let mut result = programs::Program::new(&self.name, &self.description);
        for s in &self.program {
             result.add_step(s.Contents());
        }
        result
    }
    /// Look up a program by name in the databse.
    /// 
    pub fn find(db: &Database, name : &str) -> Result<Option<Program>, rusqlite::Error> {
        //  This query should fetch a  program and all of its steps.
        let query  = "
           SELECT Programs.id, name, descripton, Steps.id, target, ramp_rate, hold_time
           FROM Programs
           INNER JOIN Steps ON Programs.id = Steps.program_id
           WHERE name = ?1
           ORDER BY step_no ASC
        ";
        let mut stmt = db.connection.prepare(query, )?;
        let mut rows = stmt.query((name,))?;
        let mut num_rows = 0;

        // This are picked out from each row:

        let mut program_id : u32 = 0;
        let mut program_name  = String::new();
        let mut description   = String::new();
        let mut steps = Vec::<Step>::new();

        while let Some(row) = rows.next()? {
            program_id = row.get_unwrap(0);
            program_name = row.get_unwrap(1);
            description = row.get_unwrap(2);

            let step_id = row.get_unwrap(3);
            let rate = row.get_unwrap(5);    // Need to convert into RampRate:
            let ramp  = if rate == -1.0 {
                programs::RampRate::AFAP
            } else {
                programs::RampRate::DegreesPerHour(rate)
            };
            let step = programs::Step::new(row.get_unwrap(4), ramp, row.get_unwrap(6));
            steps.push(Row::new(step_id, step));
        }
        let res = if steps.len() == 0 {
            return  Ok(None)
        } else {
            return Ok(Some(Program::new(program_id, &program_name, &description, &steps)))
        };
        res
    }
}