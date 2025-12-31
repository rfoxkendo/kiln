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
/// 
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FiringStep {
    id  :u64,
    sequence_id : u64,
    ramp_rate : i32,
    target_temp : u32
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
enum DatabaseError {
    SqlError(rusqlite::Error),
    Unimplemented,
}

impl Display for DatabaseError {
 fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
        DatabaseError::SqlError(e) => write!(f, "{}", e),
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
            r#" CREATE TABLE IF NOT EXISTS Kilns (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            name         TEXT        -- name of kiln.
            description  TEXT -- describes the kiln.
         )"#, 
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

    /// create a new database or open an existing one
    /// If necessary, the schema described in  the module
    /// comments are created.
    ///
    ///  ### Parameters:
    /// *  path : &str - the path to the database file.
    ///  ### Returns:
    ///   Result<KilnDatabase, Error>
    fn new(path : &str) -> result::Result<KilnDatabase, DatabaseError> {
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
        };
        
    } 
}

#[cfg(test)]
mod KilnDatabaseTests {
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
}
