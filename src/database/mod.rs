//! This module provides the database definition
//! and implementation APIs for the kiln manager.
//! There are several objects of note:
//! 
//! *  Kilns - a kiln is the devicde in which glass projects
//!    are fired.  The same firing in different kilns can result
//!    in slightly different results.
//! *  Firing sequences - these belong to kilns and
//!    consist of steps, each with a ramp speed and a target temperture.
//!    note that in this project, temperatures are degrees Fahrenheit and
//!    ramp speeds are in degrees F/sec.  A special ramp speed AFAP
//!    means that the ramp is "as fast as  possible".  For ramping down,
//!    this normally means the heating element is always off during the
//!    ramp.  For upwards ramps, the heating element is always on for AFAP ramps.
//! *  Projects are firings done on a kiln to produce a result.  Note that a
//!    project:
//!     -  Is associated with one or more firings (firing sequences)
//!     -  Is associated with zero or more pictures that are intended to
//!        capture the progress or result of the project.
//!     -  Since firing sequencdes belong to kilns, a project may use more
//!        than one kiln.
//! 

use serde::{Deserialize, Serialize};
use serde_rusqlite::*;
use std::fmt::{Display, Result, Formatter};
use std::result;
use rusqlite::Error;

/// This structure  represents a Kiln. In Sqlite, it will
/// be represented as:
/// ```sql
/// CREATE TABLE IF NOT EXISTS Kilns (
///   id           INTEGER PRIMARY KEY AUTOINCREMENT,
///   name         TEXT        -- name of kiln.
///   description  TEXT -- describes the kiln.
/// )
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Kiln {
    id : u64,
    name : String,
    description : String
}
impl Kiln {
    pub fn new(id : u64, name : &str, description : &str) -> Kiln {
        Kiln {
            id: id, 
            name: String::from(name),
             description: String::from(description)
        }
    }
    // Accessors:
    /// Note that the id is considered immutable.
    pub fn id(&self) -> u64 { 
        self.id
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }

    // Mutators:

    pub fn set_name(&mut self, new_name : &str) {
        self.name = String::from (new_name);
    }
    pub fn set_description(&mut self, new_description : &str) {
        self.description = String::from (new_description);
    }

}

///  These structures represent a firing sequence
///  and its steps.
///  In the database these are:
/// 
/// ```sql
/// CREATE TABLE IF NOT EXISTS Firing_sequences (
///   id           INTEGER  PRIMARY KEY AUTOINCREMENT,
///   name        TEXT,  
///   descripton  TEXT,
///   kiln_id     INTEGER -- Foreign key into Kilns
/// )
/// CREATE TABLE IF NOT EXISTS Firing_steps (
///    id          INTEGER PRIMARY KEY AUTOINCREMENT,
///    sequence_id INTEGER,  -- FK to Firing_sequences
///    ramp        INTEGER, -- -1 means AFAP.
///    target      INTEGER
/// )
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FiringSequence {
    id : u64,
    name : String,
    description : String,
    kiln_id : u64,
}
/// The steps in a firing sequence:
/// Note to simplify the database storage, ramp_rate is
/// -1 if the ramp is to be AFAP.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FiringStep {
    id  :u64,
    sequence_id : u64,
    ramp_rate : i32,     
    target_temp : u32,
    dwell_time  : u32    // Minutes to hold here.  
}
#[derive(Clone, PartialEq, Debug)]
enum RampRate {
    DegPerSec(u32),
    AFAP                 // As fast as possible
}
impl Display for RampRate {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RampRate::DegPerSec(n) => write!(f, "{}", n),
            RampRate::AFAP => write!(f, "AFAP")
        }
    }
}
/// This convenience struct holds a full
/// kiln program:
/// 
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct KilnProgram {
    kiln : Kiln,
    sequence : FiringSequence,
    steps    : Vec<FiringStep>
}

// Implementations of all the things that make up a kiln program:

impl FiringSequence {
    /// Creates a firing sequence on a given kiln (the id of the kiln must be
    /// known):
    /// ### Parameters:
    /// *  id - id of the firing sequence usually fetched  from the database.
    /// *  name - name of the firing sequence.
    /// *  description - what the firing sequence is for.
    /// *  kiln_id - The id of the kiln on which the sequencde is defined.  Note
    /// that the kiln_id will normally be gotten from the database by fetching the kiln.
    /// 
    /// ### Returns
    /// A FiringSequence struct.
    pub fn new(id: u64, name : &str, description : &str, kiln_id : u64) -> FiringSequence {
        FiringSequence {
            id: id, 
            name : name.into(),
            description : description.into(),
            kiln_id : kiln_id
        }
    }
    // Selectors:

    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
    pub fn kiln_id(&self) -> u64 {
        self.kiln_id
    }
    // Mutators Note the id and kiln_id are immutable.

    pub fn set_name(&mut self, new_name : &str) {
        self.name = new_name.into();
    }
    pub fn set_description(&mut self, new_description : &str) {
        self.description = new_description.into();
    }
}

impl FiringStep {
    /// Create a firing step...A firing step is one step in a
    /// kiln program.
    /// 
    /// ### Parameters
    /// *  id - id (usually gotten from the database)
    /// *  sequence - The sequencde this belongs to (which FiringSequence).
    /// *  rate  - How fast to ramp.
    /// *  target - The target temperature.
    /// 
    /// ### Returns
    /// FiringStep.
    pub fn new(id : u64, sequence : u64, rate : RampRate, target : u32, dwell : u32) -> FiringStep {
        let ramp_rate = match  rate {
            RampRate::DegPerSec(r) => r as i32,
            RampRate::AFAP => -1         // Flag for AFAP.
        };

        FiringStep {
            id : id, 
            sequence_id : sequence,
            ramp_rate : ramp_rate,
            target_temp : target,
            dwell_time : dwell
        }
    }
    // Selectors:

    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn sequence_id(&self) -> u64 {
        self.sequence_id
    }
    pub fn ramp_rate(&self) -> RampRate {
        if self.ramp_rate >= 0 {
            RampRate::DegPerSec(self.ramp_rate as u32)
        } else {
            RampRate::AFAP
        }
    }
    pub fn target_temp(&self) -> u32 {
        self.target_temp
    }
    pub fn dwell_time(&self) ->u32 {
        self.dwell_time
    }

    // Mutators.  Note that id and sequence_id are immutable.

    pub fn set_ramp_rate(&mut self, new_rate : RampRate) {
        match new_rate {
            RampRate::DegPerSec(n) => self.ramp_rate = n as i32,
            RampRate::AFAP => self.ramp_rate = -1
        };
    }
    pub fn set_target_temp(&mut self, new_target : u32) {
        self.target_temp = new_target;
    }
    pub fn set_dwell_time(&mut self, new_dwell_time : u32) {
        self.dwell_time = new_dwell_time;
    }
    
}
impl KilnProgram {
    /// Create a new, empty kiln program.  A kil program is a firing sequencde
    /// defined within a kiln.  A such it has a kiln description, a firing sequencde
    /// description and associated firing steps.
    /// 
    /// ### Parameters
    /// *  kiln     - the kiln that has the firing sequence.
    /// *  sequence - the description of the firing sequence.
    pub fn new(kiln: &Kiln, sequence: &FiringSequence) -> KilnProgram {
        KilnProgram {
            kiln: kiln.clone(),
            sequence: sequence.clone(),
            steps : Vec::new()
        }
    }
    // selectors - note these return clones...
    // There are no mutators other than the methods that add steps.
    pub fn kiln(&self) -> Kiln {
        self.kiln.clone()
    }
    pub fn sequence(&self) -> FiringSequence {
        self.sequence.clone()
    }
    pub fn steps(&self) -> Vec<FiringStep> {
        self.steps.clone()
    }
    /// Number of steps in the program.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Add a single step to the program.  Note we assume that the ids in the step are correct.
    /// 
    /// ### Parameters
    /// *  step - a Step to add to the program.
    
    pub fn add_step(&mut self, step : &FiringStep) {
        self.steps.push(step.clone());
    }
    /// add Several steps:
    /// 
    /// ### Parameters:
    /// * steps the steps to add.
    
    pub fn add_steps(&mut self, steps: &Vec<FiringStep>) {
        for step in steps {
            self.add_step(step);
        }
    }
    ///
    /// Remove a step from a program given its step number.
    /// 
    /// ### Parameters
    /// * step - step number to remove
    /// ### Returns
    /// Result<(), DatabaseError>  InvalidIndex is the only Error that can be returned.
    
    pub fn remove_step(&mut self, step : usize) -> result::Result<(), DatabaseError> {
        if step < self.steps.len() {
            self.steps.remove(step);
            Ok(())
        } else {
            Err(DatabaseError::InvalidIndex(step))
        }
    }
    ///
    /// Insert a step at a specific position in the step list.
    /// 
    /// ### Parameters:
    /// * step - reference to the step to clone into position.
    /// * index - Where to put the step.  0 means at the beginning and
    ///           len at the end.
    /// 
    /// ### Returns
    /// Result<(), DatabaseError> InvalidIndex is the only error that can be returned.
    
    pub fn insert_step(&mut self, step : &FiringStep, index : usize) -> result::Result<(), DatabaseError> {
        if index <= self.steps.len() {
            self.steps.insert(index, step.clone());
            Ok(())
        } else {
            Err(DatabaseError::InvalidIndex(index))
        }
    }
}

/// A project is a set of firing sequences
/// and an optional set of images.
/// The tables used to represent this in the
/// database are:
/// 
/// ```sql
/// CREATE TABLE IF NOT EXISTS Projects (
///    id          INTEGER PRIMARY KEY AUTOINCREMENT,
///    name        TEXT,
///    description TEXT
/// )
/// CREATE TABLE IF NOT EXISTS Project_firings (
///    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
///    project_id         INTEGER -- FK to Project.
///    firing_sequence_id INTEGER, -- FK to Firing_squences
///    comment            TEXT  -- maybe why this firing.
/// )
/// CREATE TABLE Project_images (
///   id         INTEGER PRIMARY KEY AUTOINCREMENT,
///   project_id INTEGER -- FK to project.
///   name       TEXT,   -- Original filename e.. final.jpg
///   caption    TEXT, -- What the picture is.
///   contents   BLOB -- The image file contents.
/// )
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    id : u64,
    name : String,
    description : String
}
/// The firing steps associated with a project:
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ProjectFiringSteps {
    id : u64,
    project_id : u64,
    firing_sequence_id : u64,
    comment : String
}

/// A picture associated with a project:
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ProjectImage {
    id : u64,
    project_id : u64,
    nme : String,
    description : String,
    contents : Vec<u8>
}

/// A project fully unpacked from the database:
/// 
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct KilnProject {
    project : Project,
    firings : Vec<KilnProgram>,
    pictures : Vec<ProjectImage>
}

/// This enum is the set of errors that can occur.
/// 
#[derive(Debug)]
pub enum DatabaseError {
    SqlError(rusqlite::Error),
    DuplicateName(String),
    NoSuchName(String),
    InvalidIndex(usize),
    FailedDeserialization(String),
    Unimplemented,
}

impl Display for DatabaseError {
 fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
        DatabaseError::SqlError(e) => write!(f, "{}", e),
        DatabaseError::DuplicateName(name) => write!(f, "Duplicate name: {}", name),
        DatabaseError::NoSuchName(name) => write!(f, "No such name: {}", name),
        DatabaseError::InvalidIndex(n) => write!(f, "Invalid index {}", n),
        DatabaseError::FailedDeserialization(s) => write!(f, "Failed to deserialize a {}", s),
        DatabaseError::Unimplemented => write!(f, "This operation is not yet implemented")
    }
 }   
}

/// Provides methods and access to a kiln database.
/// 
pub struct KilnDatabase {
    db :rusqlite::Connection
}

impl KilnDatabase {
    // This is a bit longer a function than I'd like.  It creates the schema
    // for the database... if it's not yet created.

    fn make_schema(db: &mut rusqlite::Connection) -> result::Result<(), rusqlite::Error> {
        // Kilns:
        if let Err(e) = db.execute(
            " CREATE TABLE IF NOT EXISTS Kilns (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            name         TEXT,        -- name of kiln.
            description  TEXT -- describes the kiln.
         )", 
            []
        ) {
            return Err(e);
        }
        // Firing_sequences:
        if let Err(e) = db.execute(
            "CREATE TABLE IF NOT EXISTS Firing_sequences (
                    id           INTEGER  PRIMARY KEY AUTOINCREMENT,
                    name        TEXT,  
                    description  TEXT,
                    kiln_id     INTEGER -- Foreign key into Kilns
                )",
            []
        ) {
            return Err(e);
        }
        // Firing_steps.
        if let Err(e) = db.execute(
            " CREATE TABLE IF NOT EXISTS Firing_steps (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    sequence_id INTEGER,  -- FK to Firing_sequences
                    ramp       INTEGER, -- -1 means AFAP.
                    target     INTEGER
                )",
            []
        ) {
            return Err(e);
        }
        // Projects:
        if let Err(e) = db.execute(
            "CREATE TABLE IF NOT EXISTS Projects (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    name        TEXT,
                    description TEXT
                )",
            []
        ) {
            return Err(e);
        }
        // Project_firings 

        if let Err(e) = db.execute(
            " CREATE TABLE IF NOT EXISTS Project_firings (
                    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
                    project_id         INTEGER -- FK to Project.
                    firing_sequence_id INTEGER, -- FK to Firing_squences
                    comment            TEXT  -- maybe why this firing.
                )",
            []
        ) {
            return Err(e)
        }
        // Project_images:

        if let Err(e) = db.execute(
            "CREATE TABLE Project_images (
                    id         INTEGER PRIMARY KEY AUTOINCREMENT,
                    project_id INTEGER -- FK to project.
                    name       TEXT,   -- Original filename e.. final.jpg
                    caption    TEXT, -- What the picture is.
                    contents   BLOB -- The image file contents.
                )",
            []
        ) {
            return Err(e);
        }
        Ok(())
    }

    // Return a value from a SELECT COUNT(*) thing:
    fn get_count<P : rusqlite::Params>(&mut self, query : &str, params : P) -> u64 {
        let mut stmt = self.db.prepare(query).unwrap();
        let mut  rows = stmt.query(params).unwrap();
        let  row = rows.next().unwrap().unwrap();

        row.get_unwrap(0)
    }

    /// create a new database or open an existing one
    /// If necessary, the schema described in  the module
    /// comments are created.
    ///
    ///  ### Parameters:
    /// *  path : &str - the path to the database file.
    ///  ### Returns:
    ///    
    /// a Result that contains the new kiln data base on success.
    pub fn new(path : &str) -> result::Result<KilnDatabase, DatabaseError> {
        let result = rusqlite::Connection::open(path);
        match result {
            Ok(mut db) => {
                if let Err(e) = Self::make_schema(&mut db) {
                    return Err(DatabaseError::SqlError(e))
                }
                return Ok(KilnDatabase {db : db})
            },
            Err(e) => return {
                Err(DatabaseError::SqlError(e))
            }
        }
        
    } 
    /// Add a new kiln to the database.  Note that kiln names must be
    /// unique
    /// 
    /// ### Parameters:
    ///  *   name : name of the new kiln, must be unique.
    ///  *   description : Description of the new kiln.  Free text.
    /// ### Returns:
    /// 
    /// If successful, nothing is returned.ss
    pub fn add_kiln(&mut self, name : &str, description: &str) -> result::Result<(), DatabaseError> {
        if self.get_count("SELECT COUNT(*) FROM Kilns WHERE name = ?", [name]) != 0 {
            return Err(DatabaseError::DuplicateName(String::from(name)));
        }
        let  stmt = self.db.prepare(
            "INSERT INTO Kilns (name, description) VALUES(?, ?)"
        );
        if let Err(e) = stmt {
            print!("{}", e);
            return Err(DatabaseError::SqlError(e));
        }
        let mut stmt = stmt.unwrap();
        if let Err(e) = stmt.execute([name, description]) {
            print!("{}", e);
            Err(DatabaseError::SqlError(e))
        } else {
            Ok(())
        }
    }
    /// Get a kiln from the database by name.  This only gets the kilns, not the firing sequences.
    /// define on the kiln.
    /// 
    /// ### Parameters:
    ///   * name - name of the kiln to fetch.
    /// ### Returns:
    ///   
    /// An option containing the fetched kiln on success.  Note that None is returned if
    /// a successful query returned nothing.
    /// 
    pub fn get_kiln(&mut self, name : &str) -> result::Result<Option<Kiln>, DatabaseError> {
        let stmt = self
            .db
            .prepare("Select id, name, description FROM Kilns WHERE name = ?");
        if let Err(sqle)  = stmt {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut stmt = stmt.unwrap();
        let rows = stmt.query([name]);
        if let Err(sqle) = rows {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut rows = rows.unwrap();
        let row = rows.next();
        if let Err(sqle) = row {
            return Err(DatabaseError::SqlError(sqle));
        }

        let row = row.unwrap();
        if let None = row {
            Ok(None)
        } else {
            let row = row.unwrap();
            let result = from_row::<Kiln>(&row);
            if let Ok(k) = result {
                Ok(Some(k))
            } else {
                Err(DatabaseError::FailedDeserialization(String::from("Kiln")))
            }
            
        }


    }
    ///  List the names of all the kilns in the database.
    /// 
    /// ### Returns:
    /// 
    /// On success a vector containing the names of all kilns found.
    /// The kiln names are in ascending lexical order.
    pub fn list_kilns(&mut self) -> result::Result<Vec<String>, DatabaseError> {
        let stmt = self.db.prepare(
            "SELECT name FROM Kilns ORDER BY name ASC"
        );
        if let Err(sqle) = stmt {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut stmt = stmt.unwrap();
        let rows = stmt.query([]);
        if let Err(sqle) = rows {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut result = vec![];
        let mut rows = rows.unwrap();
        while let Ok(r) = rows.next() {
            if let Some(row) = r {
                result.push(row.get_unwrap(0));
            } else {
                break;
            }
        }
        Ok(result)
    }
    ///
    /// Add a new kiln program for a kiln by name.
    /// Note that the program is empty and has to be added to.
    /// 
    /// ### Parameters:
    /// * kiln_name - name of the kiln: must exist.
    /// * program_name - Name of the kiln program - must be unique within the kiln
    /// * program_description - Description of the new program.
    /// ### Return:
    /// result::Result<KilnProgram, DatabaseError>
    /// *  On success, returns the kiln program that was created. Normally the
    ///    caller will edit the kiln program and then invoke the 
    ///    update_kiln_program to define the program steps.
    /// * On failure the DatabaseError will describe, to some extent, what went wrong.
    
    pub fn add_kiln_program(
        &mut self, kiln_name : &str, 
        program_name : &str,
         program_description : &str) -> result::Result<KilnProgram, DatabaseError> {
        
        // The kiln must exist else NoSuchName error or whatever database error resulted
        // from the query:

        let kiln_status = self.get_kiln(kiln_name);
        if let Err(e) = kiln_status {
            return Err(e);
        }
        let kiln_opt = kiln_status.unwrap();
        if let None = kiln_opt {
     
            return Err(DatabaseError::NoSuchName(kiln_name.into()));
        }
        let kiln = kiln_opt.unwrap();       // The kiln exists but the program must not:
        let kiln_id = kiln.id().to_string();
        if self.get_count(
            "SELECT COUNT(*) FROM Firing_sequences
            WHERE name = ? AND kiln_id = ?", 
            [program_name, &kiln_id]
        ) != 0 {
            return Err(DatabaseError::DuplicateName(program_name.into()));
        }

        // Now we can create the firing sequence:

        let insert_result = self.db.execute(
            "INSERT INTO Firing_sequences (name, description, kiln_id)
                VALUES(?,?,?)",
            [program_name, program_description, &kiln_id]
        );
        if let Err(sqle) = insert_result {
            return Err(DatabaseError::SqlError(sqle));
        }
        // Get the id of the firing sequence, construct it and the kiln_program we can
        // return:
        let program_id = self.db.last_insert_rowid();
        let firing_sequence = FiringSequence::new(
            program_id as u64, program_name, program_description, kiln.id()
        );
        Ok(KilnProgram::new(&kiln, &firing_sequence))

        
    }
    /// List the names of the kin programs defineed on a kiln:
    /// 
    /// ### Paramteers:
    /// kiln - name of the kiln for which program are listed.
    /// 
    /// ### Returns:
    /// Result<Vec<String>, DatabaseError>  - on success, the vector lists the names of the
    /// programs in lexical order by name.
    /// 
    /// #### Note:
    ///    It is not an error for the kiln to not exist... in that case, a empty
    /// vector is returned.
    
    pub fn list_kiln_programs(&mut self, kiln : &str) -> result::Result<Vec<String>, DatabaseError> {
        let stmt = self.db.prepare(
            "SELECT Firing_sequences.name FROM Firing_sequences
                INNER JOIN Kilns ON Kilns.id = Firing_sequences.kiln_id
                WHERE Kilns.name = ? 
                ORDER BY Firing_sequences.name ASC"
        );
        if let Err(sqle) = stmt {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut stmt = stmt.unwrap();
        let rows = stmt.query([kiln]);
        if let Err(sqle) = rows {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut rows = rows.unwrap();
        let mut result : Vec<String> = Vec::new();
        while let Ok(r) = rows.next() {
            if let Some(row) = r {
                result.push(row.get_unwrap(0));
            } else {
                break;
            }
        }

        Ok(result)
        
    }
    /// Fetch a kiln program on a kiln.
    /// 
    /// ### Parameters:
    /// *  kiln_name - name of the kiln.
    /// *  program_name - name of the kiln program.
    /// 
    /// ### Returns:
    /// Result<Option<KilnProgram>, DatabaseError> - on success the option:
    /// * is None if there's no matching program.
    /// Note: The kiln name is verified.
    /// 
    pub fn get_kiln_program(
        &mut self, kiln_name : &str, program_name : &str) -> result::Result<Option<KilnProgram>, DatabaseError> {
        
        Err(DatabaseError::Unimplemented)
    }
    /// Update the steps associated with a kiln program.  Note this is
    /// a transaction which 
    /// -  Removes previously existing steps defined for the program.
    /// -  Inserts the new steps for the program.
    /// In the end, the ids of the steps in the KilnProgram are updated to be correct.
    /// This is done by replacing the full steps array.
    /// The user's input step ids are ignored, of course.
    /// 
    /// ### Parameters:
    /// *  program - references the kiln program to update. Note the kiln name and program name
    /// are verified.
    /// ### Returns:
    /// Result<KilnProgram, DatabaseError>  -  On success, the updated kiln program is returned....with
    /// the step ids matching the ones in the database.
    
    pub fn update_kiln_program(&mut self, program : KilnProgram) -> result::Result<KilnProgram, DatabaseError> {
        Err(DatabaseError::Unimplemented)
    }
}

#[cfg(test)]
mod kiln_database_tests {
    use super::*;
    fn has_table(db: &mut rusqlite::Connection, name: &str) -> bool {
        let stmt = db.prepare("
        SELECT COUNT(*) FROM sqlite_schema
            WHERE type = 'table' AND name =?
        ");
        let mut  stmt = stmt.unwrap();
        let mut rows = stmt.query([name]).unwrap();    // probably 0.
        let row = rows.next().unwrap().unwrap();
        let count : u64 = row.get_unwrap(0);
        count == 1
    }
    #[test]
    fn new_1() {
        let result = KilnDatabase::new(":memory:");
        assert!(result.is_ok());
    }
    #[test]
    fn schema_1() {
        let result = KilnDatabase::new(":memory:");
        let mut db = result.unwrap().db;


        // The tables should exist.

        assert!(has_table(&mut db, "Kilns"));
        assert!(has_table(&mut db, "Firing_sequences"));
        assert!(has_table(&mut db, "Firing_steps"));
        assert!(has_table(&mut db, "Projects"));
        assert!(has_table(&mut db, "Project_firings"));
        assert!(has_table(&mut db, "Project_images"));
    }
    // Tests to manipulate Kiln definitions.

    #[test]
    fn add_kiln_1() {
        // Can add a kiln to the database:

        let mut database = KilnDatabase::new(":memory:")
            .unwrap();
        let result = database.add_kiln("Kiln1", "Large-ish kiln on 120V");
        assert!(result.is_ok());

        // The database must have the kiln and only that kiln.

        let mut stmt = database.db.prepare(
            "SELECT id, name, description FROM Kilns").unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row = rows.next().unwrap();
        assert!(row.is_some());
        let row = row.unwrap();

        let id : u64 = row.get_unwrap(0);
        let name : String = row.get_unwrap(1);
        let description : String = row.get_unwrap(2);

        assert_eq!(id, 1);
        assert_eq!(name, "Kiln1");
        assert_eq!(description, "Large-ish kiln on 120V");

        // should be no other wrows:

        assert!(rows.next().unwrap().is_none());

    }
    #[test]
    fn add_kiln_2() {
        // Duplicate kiln mame is not allowed:

        let mut database = KilnDatabase::new(":memory:")
            .unwrap();
        database.add_kiln("Kiln1", "This kiln can be added").unwrap();
        let result = database
            .add_kiln("Kiln1", "This kiln can be added"); //error.
        assert!(result.is_err());
        let error = result.err().unwrap();
        if let DatabaseError::DuplicateName(s) = error {
            assert_eq!(s, "Kiln1");                   // Maybe too fragile?
        } else {
            assert!(false);
        }
    }
    #[test]
    fn get_kiln_1() {
        // Getting a nonexistent kiln is not an error, but gives a None:

        let mut database = KilnDatabase::new(":memory:")
            .unwrap();

        // Non kilns in the database so a get retursn ok(none).
        let result = database.get_kiln("Kiln1");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

    }
    #[test]
    fn get_kiln_2() {
        // Get a kiln that exists:

        let mut database = KilnDatabase::new(":memory:").unwrap();
        database.add_kiln("kiln", "Some kiln").unwrap();

        let result = database.get_kiln("kiln");
        assert!(result.is_ok());

        let kiln = result.unwrap();
        assert!(kiln.is_some());

        let kiln = kiln.unwrap();      // The kiln itself.

        assert_eq!(kiln.id(), 1);
        assert_eq!(kiln.name(), "kiln");
        assert_eq!(kiln.description(), "Some kiln");
    }
    #[test]
    fn list_kilns_1() {
        // No kilns to list initially:

        let mut db = KilnDatabase::new(":memory:").unwrap();
        let result = db.list_kilns();

        assert!(result.is_ok());
        let listing = result.unwrap(); 
        assert_eq!(listing.len(), 0);
    }
    #[test]
    fn list_kilns_2() {
        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("SecondKiln", "This should list second").unwrap();
        db.add_kiln("FirstKiln", "This should list first").unwrap();

        let result = db.list_kilns().unwrap();
        assert_eq!(result.len(), 2);               // I added 2 kilns.

        // THe kilns come out in lexical order:

        assert_eq!(result[0], "FirstKiln");
        assert_eq!(result[1], "SecondKiln");

    }
    // Tests to manipulate firing sequences in a kiln.

    #[test]
    fn add_program_1() {
        // Successful addition:

        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let result = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            );
        
        let program = result.unwrap();    // Will give a nice error on failure.
        let kiln = program.kiln();
        let seq = program.sequence();
        
        assert_eq!(program.len(), 0);     // No steps.

        assert_eq!(kiln.name(), "Test Kiln");
        assert_eq!(kiln.description(), "My test kiln");

        assert_eq!(seq.name(), "Test");
        assert_eq!(seq.description(), "A test program");

    }
    #[test]
    fn add_program_2() {
        // Invalid kiln name gives NoSuchName error with the bad kil name:

        let mut db = KilnDatabase::new(":memory:").unwrap();
        

        let result = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ); // no such kiln.

        if let Err(e) = result {
            if let DatabaseError::NoSuchName(n) = e {
                assert_eq!(n, "Test Kiln");                     // It's the kiln that doesn't exist.
            } else {
                assert!(false, "Expected Nosuchname got {}", e);
            }
        } else {
            assert!(false, "Expected a database error");
        }

    }
    #[test]
    fn add_program_3()  {
        // Not allowed to add a duplicate program on the same kiln:

        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let result = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            );
        
        assert!(result.is_ok());
        let bad = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            );                   // Duplicate name

        if let Err(e) = bad {
            if let DatabaseError::DuplicateName(n) = e {
                assert_eq!(n, "Test");
            } else {
                assert!(false, "Expected duplicate name got {}", e);
            }
        } else {
            assert!(false, "Expected an error but was ok");
        }
    }
    #[test]
    fn add_program_4() {
        // Can add a program with a duplicate name on another kiln:

        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.
        db.add_kiln("Second", "Another kiln").unwrap();


        let result = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            );
        assert!(result.is_ok());
        let ok = db
                .add_kiln_program(
                    "Second", "Test", "A test program"
                );
        assert!(ok.is_ok());
    }
    #[test]
    fn list_programs_1() {
        // No programs in a kiln is Ok but empty:
        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let result = db.list_kiln_programs("Test Kiln");
        assert_eq!(result.unwrap().len(), 0);   // the unwrap will fail nicely if there's an error.

    }
    #[test]
    fn list_programs_2() {
        // Even if the kiln does not exist it's not an error:
        // Just empty:

        let mut db = KilnDatabase::new(":memory:").unwrap();
        assert_eq!(
            db.list_kiln_programs("something").unwrap().len(), 
            0
        );

    }
    #[test]
    fn list_programs_3() {
        // I can list all of the programs in a single kiln:

        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();
        db
            .add_kiln_program(
                "Test Kiln", "First", "Should list first"
            ).unwrap();

        let names = db.list_kiln_programs("Test Kiln").unwrap();
        assert_eq!(names.len(), 2);

        // shouild com out first then test:

        assert_eq!(names[0], "First");
        assert_eq!(names[1], "Test");
    }

    #[test]
    fn list_programs_4() {
        // Can disinguish between kilns properly:

        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.
        db.add_kiln("Second Kiln", "A second kiln").unwrap();

        db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();
        db
            .add_kiln_program(
                "Test Kiln", "First", "Should list first"
            ).unwrap();
        db.
            add_kiln_program(
                "Second Kiln",
                "Only Program", "The onlhy program in this kiln")
            .unwrap();
        
        // Two programs on the "Test Kiln" and one on the "Second Kiln"

        let names1 = db.list_kiln_programs("Test Kiln").unwrap();
        let names2 = db.list_kiln_programs("Second Kiln").unwrap();

        assert_eq!(names1.len(), 2);
        assert_eq!(names2.len(), 1);

        assert_eq!(names1[0], "First");
        assert_eq!(names1[1], "Test");

        assert_eq!(names2[0], "Only Program")

    }
}

#[cfg(test)]
mod kiln_tests {
    use super::*;
    #[test]
    fn new_1() {
        let result = Kiln::new(1, "Akiln", "A description");
        assert_eq!(
            result,
            Kiln { id: 1, name: String::from("Akiln"), description: String::from("A description")}
        );
    }
    #[test]
    fn selector_1() {
        let result = Kiln::new(1, "AKiln", "A description");
        assert_eq!(result.id(), 1);

    }
    #[test]
    fn selector_2() {
        let result = Kiln::new(1, "AKiln", "A description");
        assert_eq!(result.name(), "AKiln");
    }
    #[test]
    fn selector_3() {
        let result = Kiln::new(1, "AKiln", "A description");
        assert_eq!(result.description(), "A description")
    }

    #[test]
    fn mutator_1() {
        let mut kiln = Kiln::new(1, "AKiln", "A description");
        kiln.set_name("Kiln1");
        assert_eq!(kiln.name, "Kiln1");
    }
    #[test]
    fn mutator_2() {
        let mut kiln = Kiln::new(1, "AKiln", "A description");
        kiln.set_description("A new description for the kiln");
        assert_eq!(kiln.description, "A new description for the kiln");
    }

    
}
#[cfg(test)]
mod firing_sequence_tests {
    use super::*;
    #[test]
    fn new_1() {
        let seq = FiringSequence::new(
            123, "Slump", "Slump with no relief", 2
        );
        assert_eq!(seq,
            FiringSequence {
                id: 123, 
                name:"Slump".into(), 
                description: "Slump with no relief".into(), 
                kiln_id: 2
            }
        );
    }
    // Test selectors 
    #[test]
    fn id_1() {
        let seq = FiringSequence::new(
            123, "Slump", "Slump with no relief", 2
        );
        assert_eq!(seq.id(), 123);
    }
    #[test]
    fn name_1() {
        let seq = FiringSequence::new(
            123, "Slump", "Slump with no relief", 2
        );
        assert_eq!(seq.name(), "Slump");
    }
    #[test]
    fn description_1() {
        let seq = FiringSequence::new(
            123, "Slump", "Slump with no relief", 2
        );
        assert_eq!(seq.description(), "Slump with no relief");
    }
    #[test]
    fn kiln_id() {
        let seq = FiringSequence::new(
            123, "Slump", "Slump with no relief", 2
        );
        assert_eq!(seq.kiln_id(), 2);
    }
    #[test]
    fn set_name_1() {
        let mut seq = FiringSequence::new(
            123, "Slump", "Slump with no relief", 2
        );
        seq.set_name("Slump1");
        assert_eq!(seq.name(), "Slump1");
    }
    #[test]
    fn set_description() {
        let mut seq = FiringSequence::new(
            123, "Slump", "Slump with no relief", 2
        );
        seq.set_description("Relief slump");
        assert_eq!(seq.description, "Relief slump");
    }
}

#[cfg(test)]
mod fring_step_tests {
    use super::*;

    #[test]
    fn new_1() {
        // Degrees per second ramp rate.
        let step = FiringStep::new(
            12, 34, RampRate::DegPerSec(300), 900, 10
        );
    
    assert_eq!(
        step, FiringStep {
                id: 12, sequence_id: 34, ramp_rate: 300, target_temp: 900, dwell_time: 10
            }
        );
    }
    #[test]
    fn new_2() {
        // AFAP ramp rate:

        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900, 10
        );
        assert_eq!(
            step,  FiringStep {
                id: 12, sequence_id: 34, ramp_rate: -1, target_temp: 900, dwell_time: 10
            }
        )
    }
    // Test selectors.

    #[test]
    fn id_1() {
        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900, 10
        );
        assert_eq!(step.id(), 12);
    }
    #[test]
    fn sequence_id_1() {
        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900, 10
        );
        assert_eq!(step.sequence_id(), 34);
    }
    #[test]
    fn ramp_rate_1() {
        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900, 10
        );
        assert_eq!(step.ramp_rate(), RampRate::AFAP);
    }
    #[test]
    fn ramp_rate_2() {
        let step = FiringStep::new(
            12, 34, RampRate::DegPerSec(300), 900, 10
        );
        assert_eq!(step.ramp_rate(), RampRate::DegPerSec(300));
    }
    #[test]
    fn target_temp_1() {
        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900, 10
        );
        assert_eq!(step.target_temp(), 900);
    }
    #[test]
    fn dwell_time_1() {
        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900, 10
        );
        assert_eq!(step.dwell_time(), 10);
    }
    // Test mutators:
    #[test]
    fn set_ramp_1() {
        let mut step = FiringStep::new(
            12, 34, RampRate::AFAP, 900, 10
        );
        step.set_ramp_rate(RampRate::DegPerSec(300));
        assert_eq!(step.ramp_rate(), RampRate::DegPerSec(300));
    }
    #[test]
    fn set_ramp_2() {
        let mut  step = FiringStep::new(
            12, 34, RampRate::DegPerSec(300), 900,10
        );
        step.set_ramp_rate(RampRate::AFAP);
        assert_eq!(step.ramp_rate(), RampRate::AFAP);
    }
    #[test]
    fn set_target_1() {
        let mut step = FiringStep::new(
            12, 34, RampRate::AFAP, 900, 10
        );
        step.set_target_temp(1000);
        assert_eq!(step.target_temp(), 1000);
    }
    #[test]
    fn set_dwell_time_1() {
          let mut step = FiringStep::new(
            12, 34, RampRate::AFAP, 900, 10
        );
        step.set_dwell_time(20);
        assert_eq!(step.dwell_time(), 20);
    }

}


#[cfg(test)]
mod kiln_program_tests {
    use super::*;

    #[test]
    fn new_1() {
        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let program = KilnProgram::new(&k, &seq);

        assert_eq!(program.steps.len(), 0);
        assert_eq!(program.kiln, k);
        assert_eq!(program.sequence, seq);
    }   
    // test ability to add steps first.

    #[test]
    fn add_step_1() {
        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step = FiringStep::new(1, 1, RampRate::DegPerSec(100), 900, 10);
        program.add_step(&step);

        assert_eq!(program.steps.len(), 1);    // There's a step.
        assert_eq!(program.steps[0], step);
    }
    #[test]
    fn add_step_2() {
        // add a couple of steps:
        // 
        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);

        assert_eq!(program.steps.len(),2);    // there are 2 steps.

        assert_eq!(program.steps[0], step1);
        assert_eq!(program.steps[1], step2);
    }
    #[test]
    fn add_steps_1() {
        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        let steps = vec![step1, step2];
        program.add_steps(&steps);

        assert_eq!(program.steps.len(),2);    // there are 2 steps.

        assert_eq!(program.steps[0], steps[0]);
        assert_eq!(program.steps[1], steps[1]);
    }

    // Selectors:

    #[test]
    fn kiln_1() {
        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);

        assert_eq!(program.kiln(), k);
    }
    #[test]
    fn sequence_1() {
        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);


        assert_eq!(program.sequence(), seq);
    }
    #[test]
    fn steps_1() {
        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);

        let s = program.steps();
        assert_eq!(program.steps, s);
    }
    #[test]
    fn len_1() {
        // Initially, there are no steps:

        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let program = KilnProgram::new(&k, &seq);

        assert_eq!(program.len(), 0);
    }
    #[test]
    fn len_2() {
        // after adding steps, len is correct:

        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);

        assert_eq!(program.len(), 2);
    }
    // Program editing methods.
    #[test]
    fn remove_1() {
        // Remove invalid step:
        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);

        // Valid indices are 0,1:

        let result = program.remove_step(2);              // Invalid step:
        if let Err(e) = result {
            if let DatabaseError::InvalidIndex(n) = e {
                assert_eq!(n, 2);
            } else {
                assert!(false, "Not the right error type");
            }
        } else {
            assert!(false, "Expected Err got Ok");                     // Must be an error:
        }


    }
    #[test]
    fn remove_2() {
        // Valid removal:

        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);

        assert!(program.remove_step(0).is_ok());

        let steps = program.steps();
        assert_eq!(steps.len(), 1);    // Only one step left and...
        assert_eq!(steps[0], step2);    // It's step 2.
    }
    #[test]
    fn insert_1() {
        // Insert with an invalid index:

        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);

        let step3 = FiringStep::new(2, 1, RampRate::AFAP,100, 10);

        let result = program.insert_step(&step3, 3);  // invalid index.
        if let Err(e) = result {
            if let DatabaseError::InvalidIndex(n) = e {
                assert_eq!(n, 3);
            } else {
                assert!(false, "Incorrect error type");
            }
        } else {
            assert!(false, "Exepcted err");
        }
    }
    #[test]
    fn insert_2() {
        // Insert at beginning:

        
        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);

        let step3 = FiringStep::new(2, 1, RampRate::AFAP,100, 10);

        assert!(program.insert_step(&step3, 0).is_ok());
        assert_eq!(program.len(), 3);
        let steps = program.steps();
        assert_eq!(steps[0], step3);

    }
    #[test]
    fn insert_3() {
        // Insert at end:

        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);

        let step3 = FiringStep::new(2, 1, RampRate::AFAP,100, 10);

        assert!(program.insert_step(&step3, 2).is_ok());
        assert_eq!(program.len(), 3);
        let steps = program.steps();
        assert_eq!(steps[2], step3);

    }
    #[test]
    fn insert_4() {
        // insert in the middle between steps 0 and 1:

        let k = Kiln::new(1, "Kiln1", "The first kiln I bought");
        let seq = FiringSequence::new(1, "Slump", "Slump with no relief", 1);
        let mut program = KilnProgram::new(&k, &seq);

        let step1 = FiringStep::new(
            1, 1, RampRate::DegPerSec(100), 900, 10
        );
        let step2 = FiringStep::new(
            2, 1, RampRate::DegPerSec(300), 1200, 30
        );

        program.add_step(&step1);
        program.add_step(&step2);

        let step3 = FiringStep::new(2, 1, RampRate::AFAP,100, 10);

        assert!(program.insert_step(&step3, 1).is_ok());
        assert_eq!(program.len(), 3);
        let steps = program.steps();
        assert_eq!(steps[1], step3);
        assert_eq!(steps[0], step1);
        assert_eq!(steps[2], step2);

    }

}