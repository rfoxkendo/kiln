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
use rusqlite;

/// This stucture represents a database - it is used
/// to fetch and store data into a database. 
pub struct Database {
    connection : rusqlite::Connection
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