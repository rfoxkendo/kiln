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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FiringSequence {
    id : u64,
    name : String,
    description : String,
    kiln_id : u64,
}
/// The steps in a firing sequence:
/// Note to simplify the database storage, ramp_rate is
/// -1 if the ramp is to be AFAP.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FiringStep {
    id  :u64,
    sequence_id : u64,
    ramp_rate : i32,     
    target_temp : u32
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    pub fn new(id : u64, sequence : u64, rate : RampRate, target : u32) -> FiringStep {
        let ramp_rate = match  rate {
            RampRate::DegPerSec(r) => r as i32,
            RampRate::AFAP => -1         // Flag for AFAP.
        };

        FiringStep {
            id : id, 
            sequence_id : sequence,
            ramp_rate : ramp_rate,
            target_temp : target
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
    FailedDeserialization(String),
    Unimplemented,
}

impl Display for DatabaseError {
 fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
        DatabaseError::SqlError(e) => write!(f, "{}", e),
        DatabaseError::DuplicateName(name) => write!(f, "Duplicate name: {}", name),
        DatabaseError::NoSuchName(name) => write!(f, "No such name: {}", name),
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
                    descripton  TEXT,
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
            12, 34, RampRate::DegPerSec(300), 900
        );
    
    assert_eq!(
        step, FiringStep {
                id: 12, sequence_id: 34, ramp_rate: 300, target_temp: 900
            }
        );
    }
    #[test]
    fn new_2() {
        // AFAP ramp rate:

        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900
        );
        assert_eq!(
            step,  FiringStep {
                id: 12, sequence_id: 34, ramp_rate: -1, target_temp: 900
            }
        )
    }
    // Test selectors.

    #[test]
    fn id_1() {
        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900
        );
        assert_eq!(step.id(), 12);
    }
    #[test]
    fn sequence_id_1() {
        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900
        );
        assert_eq!(step.sequence_id(), 34);
    }
    #[test]
    fn ramp_rate_1() {
        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900
        );
        assert_eq!(step.ramp_rate(), RampRate::AFAP);
    }
    #[test]
    fn ramp_rate_2() {
        let step = FiringStep::new(
            12, 34, RampRate::DegPerSec(300), 900
        );
        assert_eq!(step.ramp_rate(), RampRate::DegPerSec(300));
    }
    #[test]
    fn target_temp() {
        let step = FiringStep::new(
            12, 34, RampRate::AFAP, 900
        );
        assert_eq!(step.target_temp(), 900);
    }
    // Test mutators:
    #[test]
    fn set_ramp_1() {
        let mut step = FiringStep::new(
            12, 34, RampRate::AFAP, 900
        );
        step.set_ramp_rate(RampRate::DegPerSec(300));
        assert_eq!(step.ramp_rate(), RampRate::DegPerSec(300));
    }
    #[test]
    fn set_ramp_2() {
        let mut  step = FiringStep::new(
            12, 34, RampRate::DegPerSec(300), 900
        );
        step.set_ramp_rate(RampRate::AFAP);
        assert_eq!(step.ramp_rate(), RampRate::AFAP);
    }
    #[test]
    fn set_target_1() {
        let mut step = FiringStep::new(
            12, 34, RampRate::AFAP, 900
        );
        step.set_target_temp(1000);
        assert_eq!(step.target_temp(), 1000);
    }

}