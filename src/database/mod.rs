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
pub enum RampRate {
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
    
    pub fn add_step(&mut self, step : &FiringStep)-> &mut KilnProgram
    {
        self.steps.push(step.clone());
        self
    }
    /// add Several steps:
    /// 
    /// ### Parameters:
    /// * steps the steps to add.
    
    pub fn add_steps(&mut self, steps: &Vec<FiringStep>)-> &mut KilnProgram {
        for step in steps {
            self.add_step(step);
        }
        self
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Project {
    id : u64,
    name : String,
    description : String
}
/// The firing steps associated with a project:
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProjectFiringStep {
    id : u64,
    project_id : u64,
    firing_sequence_id : u64,
    comment : String
}

/// A picture associated with a project:
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ProjectImage {
    id : u64,
    project_id : u64,
    name : String,
    description : String,
    contents : Vec<u8>
}

/// A project fully unpacked from the database:
/// 
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct KilnProject {
    project : Project,
    firing_comments : Vec<String>,  // Comments from the ProjectFiringStep(s).
    firings : Vec<KilnProgram>,
    pictures : Vec<ProjectImage>
}

impl Project {
    /// Create a new project.
    /// 
    /// ### Parameters:
    /// id - Id in the database.
    /// name of the project - should be unique
    /// description - Describes the project.
    /// 
    pub fn new(id : u64, name : &str, description : &str) -> Project {
        Project {
            id : id, 
            name : name.into(),
            description : description.into()
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

    // Mutators - we don't provide a public method to change the id as it's an immutable
    // database thing

    pub fn set_name(&mut self, new_name: &str) -> &mut Project{
        self.name = new_name.into();
        self
    }
    pub fn set_description(&mut self, new_desc : &str) -> &mut Project {
        self.description = new_desc.into();
        self
    }

}
impl ProjectFiringStep {
    ///
    /// Create a project firing step.  A project firing step is
    /// a kiln program run on a project.  Some projects may involve
    /// more than one firing.   For example, a weave will first 
    /// fire some pieces to a wavy mold (slump firing) and then,
    /// after sliding straight pieces into the molded pieces a second
    /// slump or tack step to flatten the weave.  Another example,
    /// Dishes may require a tack step to create a design followed by
    /// a slump into the dish mold.
    /// 
    /// This struct captures the database representatin of a firing step.
    /// 
    /// ### Parameters:
    /// *   id - id of the project firing step.
    /// *   project_id - id of the owning project.
    /// *   firing_sequence_id - id of the firing sequence (which points to the steps and back to the kiln).
    /// *   comment - Free text intended to describe why the firing was needed (e.g. final slump into dish mold).
    /// 
    /// ### Returns:
    /// ProjectFIringStep
    
    pub fn new(id : u64, project : u64, firing : u64, comment : &str) -> ProjectFiringStep {
        ProjectFiringStep {
            id: id,
            project_id : project,
            firing_sequence_id : firing,
            comment : comment.into()
        }
    }
    // Selectors:

    pub fn id(&self) -> u64 {
        self.id
    }   
    pub fn project_id(&self) -> u64 {
        self.project_id
    }
    pub fn firing_sequence_id(&self) -> u64 {
        self.firing_sequence_id
    }
    pub fn comment(&self) -> String {
        self.comment.clone()
    }

    // Mutators - We can only modify the comment:

    pub fn set_comment(&mut self, new_comment : &str) -> &mut ProjectFiringStep {
        self.comment = new_comment.into();
        self
    }
}

impl ProjectImage {
    ///
    /// Projects can have images associated with them. Throughout the life cycle of 
    /// a project, the artist might want to take pictures of the intermediate forms
    /// and final result.    These are stored as poroject images.
    /// 
    /// ### Parameters:
    /// * id - the image id in the database.
    /// * project - the id of the project the image is associated with
    /// * name  - Intended to be the original name of the image file.
    /// * description - a description that provides context for the image e.g. "Pattern tacked to the blank"
    //
    /// 
    /// The id and project are immutable, the name, description and contents
    ///  can be modified via mutators.  The contents are initially empty
    
    pub fn new(id : u64, project : u64, name : &str, description : &str) -> ProjectImage {
        ProjectImage {
            id : id,
            project_id : project,
            name : name.into(),
            description : description. into(),
            contents : Vec::<u8>::new()
        }
    }
    // Selectors (getters).

    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn project_id(&self) -> u64 {
        self.project_id
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
    pub fn contents(&self) -> Vec<u8> {
        self.contents.clone()
    }

    // Mutators/setters, these chain.

    pub fn set_name(&mut self, new_name : &str) -> &mut ProjectImage {
        self.name = new_name.into();
        self
    }
    pub fn set_description(&mut self, new_description : &str) -> &mut ProjectImage {
        self.description = new_description.into();
        self
    }
    pub fn set_contents(&mut self, new_contents : &Vec<u8>) -> &mut ProjectImage {
        self.contents = new_contents.clone();
        self
    }

}
impl KilnProject {
    ///
    /// A kiln project is built up incrementally first from a 
    /// project and then by adding firings and pictures too it.
    /// The firings can be edited, just as they can be in a
    /// kiln program, but once the project has been executed,
    /// I strongly recommend against altering the firings as that
    /// may cause the firings to not faithfully represent the project.
    /// Editing should only be used to 
    /// * Incrementally build up the set of firings as the project progresses
    /// * Correct errors in recording which firings were used.
    ///
    pub fn new(project : &Project) -> KilnProject {
        KilnProject {
            project : project.clone(),
            firing_comments : Vec::<String>::new(),
            firings : Vec::<KilnProgram>::new(),
            pictures : Vec::<ProjectImage>::new()
        }
    }
    // Selectors. Note these give copies of the attributes.
    // to change the use the mutators and editor methods.

    pub fn project(&self) -> Project {
        self.project.clone()
    }
    pub fn firing_comments(&self) -> Vec<String> {
        self.firing_comments.clone()
    }
    pub fn firings(&self) -> Vec<KilnProgram> {
        self.firings.clone()
    }
    pub fn pictures(&self) -> Vec<ProjectImage> {
        self.pictures.clone()
    }
    // Convenience accessors of the vectors; note the
    // mutators will ensure that firing_commands.len() == firings.len().

    pub fn num_firings(&self) -> usize {
        self.firings.len()
    }
    pub fn num_images(&self) -> usize {
        self.pictures.len()
    }
    /// Get information about a firing:
    /// 
    /// ### Parameters:
    /// * idx - firing number.
    /// ### Returns
    /// (String, KilnProgram) The string is the firing comment.
    /// 
    /// ### Note:
    /// panics if the idx is not in the range of firings.
    /// 
    pub fn firing(&self, idx : usize) -> (String, KilnProgram) {
        (self.firing_comments[idx].clone(), self.firings[idx].clone())
    }
    pub fn picture(&self, idx : usize) -> ProjectImage {
        self.pictures[idx].clone()
    }

    // Mutators:

    pub fn add_firing(&mut self, firing : &KilnProgram, comment :&str) -> &mut KilnProject {
        self.firing_comments.push(comment.into());
        self.firings.push(firing.clone());
        self
    }

    pub fn add_picture(&mut self, picture : &ProjectImage) -> &mut KilnProject {
        self.pictures.push(picture.clone());
        self
    }

    // Project editor methods:

    /// delete a firing and its associated comment.
    /// 
    /// ###  Parameters:
    /// * idx - the index of the firing to delete
    /// 
    /// ### Returns
    /// Result<(), DatabaseError>  On error, typically, the error returned is InvalidIndex
    /// because the index was out of range.
    /// 
    pub fn delete_firing(&mut self, idx : usize) -> result::Result<(), DatabaseError> {
        if idx < self.firings.len() {
            // Note that firing_comments and firings have the same len by design.

            self.firing_comments.remove(idx);
            self.firings.remove(idx);
            Ok(())
        } else {
            Err(DatabaseError::InvalidIndex(idx))
        }
        
    }
    /// Insert a new firing at the specified position.
    /// Use len() to append or better yet, add_firing().
    /// 
    /// ### Parameters:
    /// * idx     - the position at which to insert the firing.
    /// * program - the kiln program to insert as a firing.
    /// * comment - the firing comment.
    /// 
    /// ### Returns:
    /// Result<(), DatabaseError> Normally an InvalidIndex if idx is bad.
    
    pub fn insert_firing(
        &mut self, program : &KilnProgram, comment : &str, idx : usize
    ) -> result::Result<(), DatabaseError> {
        if idx <= self.firings.len() {
            self.firings.insert(idx, program.clone());
            self.firing_comments.insert(idx, comment.into());
            Ok(())
        } else {
            Err(DatabaseError::InvalidIndex(idx))
        }
        
    }
    /// Remove an image from the project.
    /// 
    /// ### Parameters:
    /// * idx - index of the image to remove.
    /// 
    /// ### Returns
    /// Result<(),DatabaseError> - Normally the error is InvalidIndex if idx is bad.

    pub fn delete_picture(&mut self, idx : usize) -> result::Result<(), DatabaseError> {
        if idx < self.pictures.len() {
            self.pictures.remove(idx);
            Ok(())
        } else {
            Err(DatabaseError::InvalidIndex(idx))
        }
    }

    ///
    /// Insert an image into the project at a specific position.
    /// 
    /// ### Parameters:
    /// * image - refernces the image to insert.
    /// * idx   - Where to insert it.
    /// 
    /// ### Returns:
    /// Result<(),DatabaseError> - Normally the error is InvalidIndex if idx is bad.
    
    pub fn insert_picture(&mut self, image : &ProjectImage, idx :usize) -> result::Result<(), DatabaseError> {
        if idx <= self.pictures.len() {
            self.pictures.insert(idx, image.clone());
            Ok(())
        } else {
            Err(DatabaseError::InvalidIndex(idx))
        }
    }
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
    NoSuchProgram((String, String)),
    InconsistentProgram((String, String)),
    InconsistentProject(String),
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
        DatabaseError::NoSuchProgram((kiln, program)) =>
            write!(f, "Kiln {} has no program named {}", kiln, program),
        DatabaseError::InconsistentProgram((kiln, seq))=>
            write!(f, "Input kiln ({}) program ({}) is inconsistent with database", kiln, seq),
        DatabaseError::InconsistentProject(n) =>
            write!(f, "Input project {} is inconsistent", n), 
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
                    target     INTEGER,
                    hold       INTEGER
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
                    project_id         INTEGER, -- FK to Project.
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
                    project_id INTEGER, -- FK to project.
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

    // For a project, get its firing steps and comments:
    // Stub for now.
    fn get_project_firing_steps(&mut self, project : &KilnProject) -> result::Result<Vec<(KilnProgram, String)>, DatabaseError> {
        let project_id = project.project.id;
        let mut result  : Vec<(KilnProgram, String)> = vec![];
        let mut names_comments : Vec<(String, String, String)> = vec![];
        {
            let stmt = self.db.prepare(
                "SELECT  Firing_sequences.name, Kilns.name, Project_firings.comment \
                    FROM Project_firings 
                    INNER JOIN Firing_sequences ON Firing_sequences.id = Project_firings.firing_sequence_id
                    INNER JOIN Kilns ON Firing_sequences.kiln_id = Kilns.id
                    WHERE Project_firings.project_id = ?
                    ORDER BY Firing_sequences.id ASC
                    "
            );
            if let Err(e) = stmt {
                return Err(DatabaseError::SqlError(e));
            }
            // Do the query and get the rows if there's success.  For
            // each row we get the associated kiln program and add it tothe result vector with the commment.

            let mut stmt = stmt.unwrap();                  // must succeed.
            let rows = stmt.query([project_id]);
            if let Err(e) = rows {
                return Err(DatabaseError::SqlError(e));
            }
            let mut rows = rows.unwrap();
            

            while let Ok(row) = rows.next() {
                if let Some(r) = row {
                    let fs_name : String = r.get_unwrap(0);
                    let kiln_name :String = r.get_unwrap(1);
                    let comment : String = r.get_unwrap(2);

                    names_comments.push((kiln_name.clone(), fs_name.clone(), comment.clone()));


                } else {
                    break;                   // End of iteration?
                }
            }
        }
        // Now get the kiln programs:

        for (kname, pname, comment) in names_comments {
            let pgm = self.get_kiln_program(&kname, &pname);
            if let Err(e) = pgm {
                return Err(e);
            }
            let pgm = pgm.unwrap().unwrap();        // None is not an option given the query.
            result.push((pgm.clone(), comment.clone()));
        }

        Ok(result)
    }
    // For a project, get its images
    // 
    fn get_project_images(&mut self, project: &KilnProject) -> result::Result<Vec::<ProjectImage>, DatabaseError> {
        let stmt = self.db.prepare("
            SELECT id, project_id, name, caption, contents, caption as description FROM Project_images
            WHERE project_id = ?  ORDER BY id ASC
        ");
        if let Err(sqle) = stmt {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut stmt = stmt.unwrap();
        let rows = stmt.query([project.project().id()]);
        if let Err(sqle) = rows {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut rows = rows.unwrap();
        let mut result = Vec::<ProjectImage>::new();
        while let Ok(row) = rows.next() {
            if let Some(r) = row {
                let image = from_row::<ProjectImage>(&r);
                if let Err(e) = image {
                    return Err(DatabaseError::FailedDeserialization(format!("{} : {}", "Project Image", e)));
                }
                result.push(image.unwrap());
            } else {
                break;
            }
        }

        Ok(result)
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
    /// Result<Vec&lt;String&gt; DatabaseError>;  - on success, the vector lists the names of the
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
    /// Result<Option&lt;KilnProgram&gt;, DatabaseError> - on success the option:
    /// * is None if there's no matching program.
    /// Note: The kiln name is not verified.
    /// 
    pub fn get_kiln_program(
        &mut self, kiln_name : &str, program_name : &str) -> result::Result<Option<KilnProgram>, DatabaseError> {
        
        // Get the definition without the steps:

        let stmt = self.db.prepare(
            "SELECT Kilns.id, Kilns.description, 
                    Firing_sequences.id, Firing_sequences.description
                 FROM Kilns
                 INNER JOIN Firing_sequences ON Firing_sequences.kiln_id = Kilns.id
                 WHERE Kilns.name = ? AND Firing_sequences.name = ?"
        );
        if let Err(sqle) = stmt {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut stmt = stmt.unwrap();
        let rows = stmt.query([kiln_name, program_name]);
        if let Err(sqle) = rows {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut rows = rows. unwrap();
        let row = rows.next();                              // Only one match allowed...
        if let Err(sqle) = row {
            return Err(DatabaseError::SqlError(sqle));
        }
        let row = row.unwrap();
        if let None = row {
            return Ok(None);
        }
        let row = row.unwrap();

        // Deser can't do such a nice compound I think?
        let kiln_desc : String = row.get_unwrap(1);
        let kiln = Kiln::new(
            row.get_unwrap(0), kiln_name, &kiln_desc
        );
    
        let fs_desc : String = row.get_unwrap(3);
        let program = FiringSequence::new(
            row.get_unwrap(2), &program_name, &fs_desc, kiln.id()
        );
        let mut kiln_program = KilnProgram::new(&kiln, &program);

        // Now we need to fetch rows firing_steps with our program id.
        // ASs are needed to make serde able to deserialize :

        let stmt = self.db.prepare(
            "SELECT id, sequence_id, 
                            ramp AS ramp_rate, 
                            target AS target_temp,
                             hold AS dwell_time 
                    FROM Firing_steps
                    WHERE sequence_id = ? ORDER BY id ASC"
        );
        if let Err(sqle) =stmt {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut stmt = stmt.unwrap();
        let rows = stmt.query([program.id()]);
        if let Err(sqle) = rows {
            return Err(DatabaseError::SqlError(sqle));
        }
        let mut rows = rows.unwrap(); 
        while let Ok(r) = rows.next() {
            if let Some(row) = r {
                let step =  from_row::<FiringStep>(&row);
                if let Err(_) = step {
                    return Err(DatabaseError::FailedDeserialization("FiringStep".into()));
                }
                // Add the step to the program:
                let step = step.unwrap();
                kiln_program.add_step(&step);
            } else {
                break;                   // NO more rows.ss
            }
        }

        Ok(Some(kiln_program))

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
    /// 
    /// ### Notes:
    /// *  The kiln name and id are validated.
    /// *  The kiln program name and Id are validated.
    
    pub fn update_kiln_program(&mut self, program : &KilnProgram) -> result::Result<KilnProgram, DatabaseError> {
        // Validate the kiln:

        let kiln= program.kiln();
        let seq = program.sequence();

        let current_program = self.get_kiln_program(&kiln.name(), &seq.name());
        if let Err(e) = current_program {
            return Err(e);
        }
        let current_program =current_program.unwrap();
        if let None = current_program {
            return Err(DatabaseError::NoSuchProgram((kiln.name(), seq.name())));
        }
        let current_program = current_program.unwrap();

        // The kiln and sequence must match the new program info (not the steps):
        // @TODO:  Support changing the descriptions.
        if kiln != current_program.kiln() || seq != current_program.sequence() {
            return Err(DatabaseError::InconsistentProgram((kiln.name(), seq.name())));
        }
        
        // start a transaction if there are errors we'll abort it:

        let t = self.db.transaction();
        if let Err(sqle) =t {
            return Err(DatabaseError::SqlError(sqle));
        } 
        let mut t = t.unwrap();
        t.set_drop_behavior(rusqlite::DropBehavior::Rollback);
        // Kill off the existing steps:

        let existing_steps = current_program.steps();
        let mut existing_step_ids = Vec::<u64>::new();
        for step in existing_steps {
            existing_step_ids.push(step.id());
        }
        let del_sql= format!("DELETE FROM Firing_steps WHERE id in({})",
            existing_step_ids.iter().map(|_| String::from("? ")).collect::<Vec<String>>().join(", "));
        let del_status = t.execute(
            &del_sql,
            rusqlite::params_from_iter(existing_step_ids)
        );
        if let Err(sqle) = del_status {
            return Err(DatabaseError::SqlError(sqle));
        }
        // Add in the new rows

        let mut resulting_program = KilnProgram::new(&kiln, &seq);    // So we can modify the step ids.
        {
            let step_sql = t.prepare (
                "INSERT INTO Firing_steps (sequence_id, ramp, target, hold) 
                    VALUES (?, ?, ?, ?)"
            );
            if let Err(sqle) = step_sql {
                return Err(DatabaseError::SqlError(sqle));
            }
            let mut step_sql = step_sql.unwrap();
            let seqid = seq.id();                      // Notationally convenient.
            for step in program.steps() {
                let ramp = if let RampRate::DegPerSec(n) = step.ramp_rate() {
                    n as i32
                } else {
                    -1
                };
                let status = step_sql.execute(
                    [seqid as i64, ramp as i64 , step.target_temp() as i64 , step.dwell_time() as i64]
                );
                if let Err(sqle) = status {

                    return Err(DatabaseError::SqlError(sqle));
                } else {
                    let final_step = FiringStep::new(
                            t.last_insert_rowid() as u64, seqid, step.ramp_rate(), step.target_temp(), step.dwell_time()
                        );
                    resulting_program.add_step(&final_step);
                }

            }
        }
        if let Err(sqle) = t.commit() {
            return Err(DatabaseError::SqlError(sqle));
        }     // 'must' succeed but...
        Ok(resulting_program)
    }
    // Suport for projects.

    /// Add a new project to the database.  A project is a set of firing steps
    /// which are intended to achieve a finished piece. Projects are added
    /// to the database empty.  Later, firing steps and images can be added
    /// to a project.
    /// 
    /// ### Parameters:
    /// * name - a unique name for the project.
    /// * description - a description of the project.
    /// 
    /// ### Returns:
    /// Result<KilnProject, DatabaseError> - On success the empty kiln project is returned.
    /// 
    
    pub fn add_project(&mut self, name : &str, description : &str) -> result::Result<KilnProject, DatabaseError> {

        // Check the uniqueness of the project name:

        if self.get_count("SELECT COUNT(*) FROM Projects WHERE name=?",[name]) > 0 {
            return Err(DatabaseError::DuplicateName(name.into()));
        }
        // Make the project in the database.

        let status = self.db.execute(
            "INSERT INTO Projects (name, description) VALUES (?,?)",
            [name, description]
        );
        if let Err(sqle) = status {
            return Err(DatabaseError::SqlError(sqle));
        }
        // Success so make the project and the KilnProject:

        let project = Project::new(self.db.last_insert_rowid() as u64, name, description);
        Ok(KilnProject::new(&project))

    
    }
    
    ///
    /// Return the full project definition given a project name.
    /// 
    /// ### Parameters:
    /// * name -name of the project.
    /// 
    /// ### Returns:
    /// 
    /// Result<Option<KilnProject>, DatabaseError> :
    /// *  Error if unable to execute the queries.
    /// *  None if there's no such project.
    /// *  Some containing the project definition.
    
    pub fn get_project(&mut self, name : &str) -> result::Result<Option<KilnProject>, DatabaseError> {
        // This block allows the root_query to drop which prevents us from havig an immutable
        // borrow of self when we need mutable borrows to get the firings and images later on.

        let mut full_project = {
            let root_query = self.db.prepare(
                "SELECT id, name, description FROM Projects 
                WHERE name = ?"
            );
            if let Err(sqle) = root_query {
                return Err(DatabaseError::SqlError(sqle));
            }
            let mut root_query = root_query.unwrap();
            let root_rows = root_query.query([name]);
            if let Err(sqle) = root_rows {
                return Err(DatabaseError::SqlError(sqle));
            }
            let mut  root_rows = root_rows.unwrap();
            let row = root_rows.next();
            if let Err(sqle) = row {
                return Err(DatabaseError::SqlError(sqle));
            }
            let row = row.unwrap();
            if let None = row {
                return Ok(None);
            }
            let row = row.unwrap();
            let project = from_row::<Project>(&row);
            if let Err(e) = project {
                return Err(DatabaseError::FailedDeserialization(format!("{} : {}", "Project", e)));
            }

            KilnProject::new(&project.unwrap())
        };

        // Fold in the firings:

        let firings = self.get_project_firing_steps(&full_project);
        if let Err(e) = firings {
            return Err(e);
        }
        for firing in firings.unwrap() {
            full_project.add_firing(&firing.0, &firing.1);
        }
        // Fold in the images:

        let images = self.get_project_images(&full_project);
        if let Err(e) = images {
            return Err(e);
        }
        for image in images.unwrap() {
            full_project.add_picture(&image);
        }
        Ok(Some(full_project))
    }


    ///
    /// Add a new firing for a project.  A firing is  kiln program defined on a kiln.
    /// It's a project step.  A project may require more than one firing.  For example,
    /// A simple dish might require a tack step to add a design to the blank and then a slump
    /// step into the mold 
    /// 
    /// ### Parameters:
    /// * project - the kiln project we're adding a step to.
    /// * kiln    - Name of the kiln we're firing in.
    /// * program - Name of that kiln's firing program used for the Firing.
    /// * comment - Might give the reason for this firing.
    /// 
    /// ### Returns:
    /// 
    /// Result<KilnProject, DatabaseError> - On success, the updtaed kiln program.
    /// 
    pub fn add_project_firing(
        &mut self, project : &KilnProject, kiln_name : &str, program_name : &str, comment : &str
    ) -> result::Result<KilnProject, DatabaseError> {
        let program = self.get_kiln_program(kiln_name, program_name);
        if let Err(e) = program {
            return Err(e);
        }
        let program = program.unwrap();
        if let None = program {
            return Err(DatabaseError::NoSuchProgram((kiln_name.into(), program_name.into()))); 
        }
        let program = program.unwrap();
        
        // The database project must exist and must match our input project.

        let db_project = self.get_project(&project.project().name());
        if let Err(e) = db_project {
            return Err(e);
        }
        let db_project = db_project.unwrap();
        if let None = db_project {
            return Err(DatabaseError::NoSuchName(project.project().name()));
        }
        let db_project = db_project.unwrap();

        if db_project.project().id() != project.project().id() {
            return Err(DatabaseError::InconsistentProject(project.project().name()));
        }
        // Add the program to the project in the database.

        let result = self.db.execute(
            "INSERT INTO Project_firings (project_id, firing_sequence_id, comment)
                        VALUES(?,?,?)
            ",
            [project.project.id().to_string(), program.sequence().id().to_string(), comment.into()]
        );
        if let Err(sqle) = result {
            return Err(DatabaseError::SqlError(sqle));
        }
        // Update the project from the database:

        let updated_project = self.get_project(&project.project.name);
        if let Err(e) = updated_project {
            return Err(e);
        }
        let updated_project = updated_project.unwrap().unwrap();      // Must work.
        Ok(updated_project)


    }
    /// Add an image to a project.
    /// 
    /// ### Parameters
    ///    project - references the kiln project to modify as it is so far.
    ///    image_name  - Name of an image to add... this is usually the name of the file from which the data came.
    ///    description - Description of the image (e.g. "After initial cuttin before fring with Tack fuse").
    ///    image_data  - The data that makes up the image.
    /// 
    /// ### Returns
    /// Result<KilnProject, DatabaseError> - the updated project on success.
    /// 
    pub fn add_project_image(
        &mut self, project : &KilnProject,
        image_name : &str, 
        description : &str, 
        image_data : &Vec<u8>) -> result::Result<KilnProject, DatabaseError> {
        
        let project_id = project.project.id;
        let status = self.db.execute(
            "INSERT INTO Project_images
                (project_id, name, caption, contents) VALUES (?,?,?,?)
            ", (project_id, image_name, description, image_data)
        );
        if let Err(e) = status {
            return Err(DatabaseError::SqlError(e));
        }
        // Return the result of get_project on the current project name.

        let final_project = self.get_project(&(project.project.name));
        if let Err(e) = final_project {
            return Err(e);
        }

        Ok(final_project.unwrap().unwrap())
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
    // Tests for getting empty programs.
    // We need to test update_kiln_program to test get with programs that are not empty in the DB.
    #[test]
    fn get_empty_program1() {
        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        // No such program to get give me Ok(None);

        let program = db.get_kiln_program("Test Kiln", "no program");
        assert!(program.is_ok());
        let program = program.unwrap();

        assert!(program.is_none());
    }
    #[test]
    fn get_empty_program2() {
        // No such kiln also gives a None:

        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let program = db.get_kiln_program("No such", "program");
        assert!(program.is_ok());
        let program =program.unwrap();
        assert!(program.is_none());
    }
    #[test]
    fn get_empty_program3() {
        // Empty existing program:

        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let program_added = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();
        
        let program_gotten = db.get_kiln_program("Test Kiln", "Test");
        assert!(program_gotten.is_ok());
        let program_gotten = program_gotten.unwrap();
        if let Some(got) = program_gotten {
            assert_eq!(got.kiln(), program_added.kiln());
            assert_eq!(got.sequence(), program_added.sequence());
            assert_eq!(got.steps.len(), 0);
        } else {
            assert!(false, "Expected to fetch a program");
        }
    }
    #[test]
    fn update_kiln_program_1() {
        // add step ok.

        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let mut program_added = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();

        // Note the step id and seq id are gotten from the database and program respectively.
        program_added.add_step(
            &FiringStep::new(0, 0, RampRate::DegPerSec(300), 1000, 10)
        );

        // Update the program in the database with this single step:

        let updated_status = db.update_kiln_program(&program_added);
        let updated_program = updated_status.unwrap();  // Gives good errmsg.

        // The kiln and sequence should not have changed:

        assert_eq!(updated_program.kiln(), program_added.kiln());
        assert_eq!(updated_program.sequence(), program_added.sequence());
        let steps = updated_program.steps();

        // validate the single step that should be there:
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].id(), 1);    // first one added.
        assert_eq!(steps[0].sequence_id(), updated_program.sequence().id());
        assert_eq!(steps[0].ramp_rate(), RampRate::DegPerSec(300));
        assert_eq!(steps[0].target_temp(), 1000);
        assert_eq!(steps[0].dwell_time(), 10);


    }
    #[test]
    fn update_kiln_program_2() {
        // mismatch between kiln id fails add step:

        
        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let mut program_added = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();

        // Note the step id and seq id are gotten from the database and program respectively.
        program_added.add_step(
            &FiringStep::new(0, 0, RampRate::DegPerSec(300), 1000, 10)
        );
        // butcher the kiln id:

        program_added.kiln.id += 1;  // now it's a bad id.

        let update_status = db.update_kiln_program(&program_added);
        if let Err(e) = update_status {
            if let DatabaseError::InconsistentProgram((k, s)) = e {
                assert_eq!(k, "Test Kiln");
                assert_eq!(s, "Test");
            } else {
                assert!(false, "Expected inconstent program got: {}", e);
            }
        } else {
            assert!(false, "Expected an error got OK");
        }
    }
    #[test]
    fn update_kiln_program_3() {
        // Mismatching the sequence id also fails an update:

        
        
        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let mut program_added = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();

        // Note the step id and seq id are gotten from the database and program respectively.
        program_added.add_step(
            &FiringStep::new(0, 0, RampRate::DegPerSec(300), 1000, 10)
        );
        // butcher the kiln id:

        program_added.sequence.id += 1;  // now it's a bad id.

        let update_status = db.update_kiln_program(&program_added);
        if let Err(e) = update_status {
            if let DatabaseError::InconsistentProgram((k, s)) = e {
                assert_eq!(k, "Test Kiln");
                assert_eq!(s, "Test");
            } else {
                assert!(false, "Expected inconstent program got: {}", e);
            }
        } else {
            assert!(false, "Expected an error got OK");
        }
    }
    #[test]
    fn update_kiln_program_4() {
        // Changing thekiln id in the sequence results inconsistent.

        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let mut program_added = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();

        // Note the step id and seq id are gotten from the database and program respectively.
        program_added.add_step(
            &FiringStep::new(0, 0, RampRate::DegPerSec(300), 1000, 10)
        );
        // butcher the kiln id:

        program_added.sequence.kiln_id += 1;  // now it's a bad id.

        let update_status = db.update_kiln_program(&program_added);
        if let Err(e) = update_status {
            if let DatabaseError::InconsistentProgram((k,s)) = e {
                assert_eq!(k, "Test Kiln");
                assert_eq!(s, "Test");
            } else {
                assert!(false, "Expected inconsistent program  program got: {}", e);
            }
        } else {
            assert!(false, "Expected an error got OK");
        }
    }
    #[test]
    fn update_kiln_program_5() {
        // Changing the kiln name gives no such program..

        let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let mut program_added = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();

        // Note the step id and seq id are gotten from the database and program respectively.
        program_added.add_step(
            &FiringStep::new(0, 0, RampRate::DegPerSec(300), 1000, 10)
        );
        // butcher the kiln id:

        program_added.kiln.name = String::from("no such kiln");

        let update_status = db.update_kiln_program(&program_added);
        if let Err(e) = update_status {
            if let DatabaseError::NoSuchProgram((k,s)) = e {
                assert_eq!(k, "no such kiln");
                assert_eq!(s, "Test");
            } else {
                assert!(false, "Expected No Such program got: {}", e);
            }
        } else {
            assert!(false, "Expected an error got OK");
        }

    }
    #[test]
    fn update_kiln_program_6() {
        // CHangint the seq name is also bad:

         let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let mut program_added = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();

        // Note the step id and seq id are gotten from the database and program respectively.
        program_added.add_step(
            &FiringStep::new(0, 0, RampRate::DegPerSec(300), 1000, 10)
        );
        // butcher the kiln id:

        program_added.sequence.name = String::from("no such program");

        let update_status = db.update_kiln_program(&program_added);
        if let Err(e) = update_status {
            if let DatabaseError::NoSuchProgram((k,s)) = e {
                assert_eq!(k, "Test Kiln");
                assert_eq!(s, "no such program");
            } else {
                assert!(false, "Expected No Such program got: {}", e);
            }
        } else {
            assert!(false, "Expected an error got OK");
        }


    }
    #[test]
    fn update_kiln_program_7() {
        // The resulting program can be gotten properly after a step is added:

         let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let mut program_added = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();

        // Note the step id and seq id are gotten from the database and program respectively.
        program_added.add_step(
            &FiringStep::new(0, 0, RampRate::DegPerSec(300), 1000, 10)
        );
        let update_status = db.update_kiln_program(&program_added);
        assert!(update_status.is_ok());
        let updated_program = update_status.unwrap();

        let got_program = db
            .get_kiln_program("Test Kiln", "Test")
            .unwrap().unwrap();

        assert_eq!(got_program, updated_program);    // Should be the same!
    }
    #[test]
    fn update_kiln_program_8() {
        // Multiple steps are fine:

        // The resulting program can be gotten properly after a step is added:

         let mut db = KilnDatabase::new(":memory:").unwrap();
        db.add_kiln("Test Kiln", "My test kiln").unwrap(); // MUut succeeed.

        let mut program_added = db
            .add_kiln_program(
                "Test Kiln", "Test", "A test program"
            ).unwrap();

        // Note the step id and seq id are gotten from the database and program respectively.
        program_added.add_step(
            &FiringStep::new(0, 0, RampRate::DegPerSec(300), 1000, 10)
        )
        .add_step(
            &FiringStep::new(0, 0, RampRate::DegPerSec(300), 1200,  30)
        )
        .add_step(
            &FiringStep::new(0, 0, RampRate::DegPerSec(300), 1320, 10)
        )
        .add_step(
            &FiringStep::new(0, 0, RampRate::AFAP, 900, 60)
        );
        let update_status = db.update_kiln_program(&program_added);
        let updated_program = update_status.unwrap();

        let got_program = db
            .get_kiln_program("Test Kiln", "Test")
            .unwrap().unwrap();

        assert_eq!(got_program, updated_program);    // Should be the same!

    }
    // Tests for Kiln projects.

    #[test]
    fn add_project_1() {
        // success:

        let mut db = KilnDatabase::new(":memory:").unwrap();

        let result = db.add_project("Test Project", "A test Project");
        let result = result.unwrap();    // Better errror message than assert if it's not ok.

        assert_eq!(result.project().name(), "Test Project");
        assert_eq!(result.project().description(), "A test Project");

        let id = db.db.last_insert_rowid();  
        assert_eq!(result.project().id(), id as u64);

        // No firings and no images:

        assert_eq!(result.firing_comments.len(), 0);
        assert_eq!(result.firings.len(), 0);
        assert_eq!(result.pictures.len(), 0);
    }
    #[test]
    fn add_project_2() {
        // Duplicate project name is bad:

     let mut db = KilnDatabase::new(":memory:").unwrap();

        let result = db.add_project("Test Project", "A test Project");
        result.unwrap();    // Better errror message than assert if it's not ok.

        let result = db.add_project("Test Project", "A faild project insert");
        if let Err(e) = result {
            if let DatabaseError::DuplicateName(n) = e {
                assert_eq!(n, "Test Project");
            } else {
                assert!(false, "Expected duplicate name error, got : {}", e);
            }
        } else {
            assert!(false, "Expected a database error but was ok.");
        }
    }
    // Can add firings to kiln programs:

    #[test]
    fn add_firing_1() {
        let mut db = KilnDatabase::new(":memory:").unwrap();

        // Add a kiln and a firing sequence to the kiln.
        db.add_kiln("Big", "A big kiln").unwrap();
        let mut program = db.add_kiln_program("Big", "program", "A program").unwrap();
        program.add_step(&FiringStep::new(0, 0, RampRate::DegPerSec(300), 900, 10 ));
        program.add_step(&FiringStep::new(0, 0, RampRate::DegPerSec(300), 1200, 5));
        program.add_step(&FiringStep::new(0, 0, RampRate::DegPerSec(300), 1400, 10));
        program.add_step(&FiringStep::new(0, 0, RampRate::AFAP, 1000, 30));
        db.update_kiln_program(&program).unwrap();

        // Make a project and add a firing:

        let project = db.add_project("AProject", "A test project").unwrap();

        // Add the step:

        let project = 
            db.add_project_firing(&project, "Big", "program", "The first firing");
        let project = project.unwrap();
        

        // The base stuff should still be there but there also should be a firing and a riging comment.

        assert_eq!(project.project.name(), "AProject");
        assert_eq!(project.project.description(), "A test project");
        
        assert_eq!(project.firing_comments.len(), 1);  
        assert_eq!(project.firing_comments[0], "The first firing");

        assert_eq!(project.firings.len(), 1);

        let firing = project.firings[0].clone();
        assert_eq!(firing.steps.len(), 4);      // The firing consists of 4 steps.

        // Since the ids won't match we need to do this the hard way

        assert_eq!(firing.steps[0].ramp_rate(), RampRate::DegPerSec(300));
        assert_eq!(firing.steps[0].target_temp(), 900);
        assert_eq!(firing.steps[0].dwell_time(), 10);

        assert_eq!(firing.steps[1].ramp_rate(), RampRate::DegPerSec(300));
        assert_eq!(firing.steps[1].target_temp(), 1200);
        assert_eq!(firing.steps[1].dwell_time(), 5);

        assert_eq!(firing.steps[1].ramp_rate(), RampRate::DegPerSec(300));
        assert_eq!(firing.steps[2].target_temp(), 1400);
        assert_eq!(firing.steps[2].dwell_time(), 10);

        assert_eq!(firing.steps[3].ramp_rate(), RampRate::AFAP);
        assert_eq!(firing.steps[3].target_temp(), 1000);
        assert_eq!(firing.steps[3].dwell_time(), 30);
    }
    #[test]
    fn add_firing_2() {
        // No such firing sequence...

        let mut db = KilnDatabase::new(":memory:").unwrap();

        // Add a kiln and a firing sequence to the kiln.
        db.add_kiln("Big", "A big kiln").unwrap();
        let project = db.add_project("AProject", "A test project").unwrap();

        // Add the step:

        let project = 
            db.add_project_firing(&project, "Big", "program", "The first firing");
        
        assert!(project.is_err());
    }
    #[test]
    fn add_firing_3() {
        // can add more than one firing sequence to the project.
        let mut db = KilnDatabase::new(":memory:").unwrap();

        // Add a kiln and a firing sequence to the kiln. Kinda like a full fuse.
        db.add_kiln("Big", "A big kiln").unwrap();
        let mut program = db.add_kiln_program("Big", "program", "A program").unwrap();
        program.add_step(&FiringStep::new(0, 0, RampRate::DegPerSec(300), 900, 10 ));
        program.add_step(&FiringStep::new(0, 0, RampRate::DegPerSec(300), 1200, 5));
        program.add_step(&FiringStep::new(0, 0, RampRate::DegPerSec(300), 1400, 10));
        program.add_step(&FiringStep::new(0, 0, RampRate::AFAP, 1000, 30));
        db.update_kiln_program(&program).unwrap();

        // And another.. kind of like a slump.
        let mut program = 
            db.add_kiln_program("Big", "second", "Simple slump")
            .unwrap();
        program.add_step(&FiringStep::new(0, 0, RampRate::DegPerSec(250), 900, 10));
        program.add_step(&FiringStep::new(0, 0, RampRate::DegPerSec(250), 1250, 30));
        program.add_step(&FiringStep::new(0,0, RampRate::AFAP, 1000, 60));
        db.update_kiln_program(&program).unwrap();


        // Make project with first a full fuse then a slump:

        let project = db
            .add_project("Compound", "A project with two firing steps")
            .unwrap();
        let project = db.add_project_firing(&project, "Big", "program", "Full fuse step").unwrap();
        let project = db.add_project_firing(&project, "Big", "second", "Slump into mold").unwrap();

        // CHeck the steps, add_firing_1 determined(?) that everything else is good.

        assert_eq!(project.firing_comments.len(), 2);  // Ezach firing has a comment and no more.
        assert_eq!(project.firing_comments[0], "Full fuse step");
        assert_eq!(project.firing_comments[1], "Slump into mold");

        let firings = &project.firings;
        assert_eq!(firings.len(), 2);
        let firing1 = firings[0].clone();
        let firing2 = firings[1].clone();

        assert_eq!(firing1.steps.len(), 4);      // The firing consists of 4 steps.

        
        
        // Since the ids won't match we need to do this the hard way

        assert_eq!(firing1.steps[0].ramp_rate(), RampRate::DegPerSec(300));
        assert_eq!(firing1.steps[0].target_temp(), 900);
        assert_eq!(firing1.steps[0].dwell_time(), 10);

        assert_eq!(firing1.steps[1].ramp_rate(), RampRate::DegPerSec(300));
        assert_eq!(firing1.steps[1].target_temp(), 1200);
        assert_eq!(firing1.steps[1].dwell_time(), 5);

        assert_eq!(firing1.steps[1].ramp_rate(), RampRate::DegPerSec(300));
        assert_eq!(firing1.steps[2].target_temp(), 1400);
        assert_eq!(firing1.steps[2].dwell_time(), 10);

        assert_eq!(firing1.steps[3].ramp_rate(), RampRate::AFAP);
        assert_eq!(firing1.steps[3].target_temp(), 1000);
        assert_eq!(firing1.steps[3].dwell_time(), 30);


        assert_eq!(firing2.steps.len(), 3);

        assert_eq!(firing2.steps[0].ramp_rate(), RampRate::DegPerSec(250));
        assert_eq!(firing2.steps[0].target_temp(), 900);
        assert_eq!(firing2.steps[0].dwell_time(), 10);

        assert_eq!(firing2.steps[1].ramp_rate(), RampRate::DegPerSec(250));
        assert_eq!(firing2.steps[1].target_temp(), 1250);
        assert_eq!(firing2.steps[1].dwell_time(), 30);

        assert_eq!(firing2.steps[2].ramp_rate(), RampRate::AFAP);
        assert_eq!(firing2.steps[2].target_temp(), 1000);
        assert_eq!(firing2.steps[2].dwell_time(), 60);



    }

    // Tests for adding images to projects.  

    #[test]
    fn add_project_image_1() {
        // add a single image to a project:

        let mut db = KilnDatabase::new(":memory:").unwrap();

        
        // Make project with first a full fuse then a slump:

        let project = db
            .add_project("Images", "A project with an image")
            .unwrap();

        let image_data :Vec<u8> = vec![0,1,2,3,4,5];
        let project = db.add_project_image(&project, "junk.png", "A junk image", &image_data);
        let project = project.unwrap();

        assert_eq!(project.pictures.len(), 1);     // There is an image -- and only one.
        let image = &project.pictures[0];
        assert_eq!(image.project_id, project.project.id);
        assert_eq!(image.name, "junk.png");
        assert_eq!(image.description, "A junk image");
        assert_eq!(image.contents, image_data);

    }
    #[test]
    fn add_project_image_2() {
        // add more than one image.

         let mut db = KilnDatabase::new(":memory:").unwrap();

        
        // Make project with first a full fuse then a slump:

        let project = db
            .add_project("Images", "A project with an image")
            .unwrap();

        let image_data1 :Vec<u8> = vec![0,1,2,3,4,5];
        let project = db
            .add_project_image(&project, "junk.png", "A junk image", &image_data1)
            .unwrap();
        
        let image_data2 : Vec<u8> = vec![5,4,3,2,1,0];
        let project = db
            .add_project_image(&project, "junk.jpg", "fake image", &image_data2)
            .unwrap();


        assert_eq!(project.pictures.len(), 2);     // There is an image -- and only one.
        let image = &project.pictures[0];
        assert_eq!(image.project_id, project.project.id);
        assert_eq!(image.name, "junk.png");
        assert_eq!(image.description, "A junk image");
        assert_eq!(image.contents, image_data1);

        let image = &project.pictures[1];
        assert_eq!(image.name, "junk.jpg");
        assert_eq!(image.description, "fake image");
        assert_eq!(image.contents, image_data2);

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
#[cfg(test)]
mod project {
    use super::*;

    #[test]
    fn new_1() {
        let project =Project::new(1, "Test Project", "A test project");
        assert_eq!(
            project, Project { id: 1, name: "Test Project".into(), description: "A test project".into()}
        );
    }
    // Test the selector/getters
    #[test]
    fn id_1() {
        let project =Project::new(1, "Test Project", "A test project");
        assert_eq!(1, project.id());
    }
    #[test]
    fn name_1() {
        let project =Project::new(1, "Test Project", "A test project");
        assert_eq!(project.name(), "Test Project");
    }
    #[test]
    fn description_1() {
        let project =Project::new(1, "Test Project", "A test project");
        assert_eq!(project.description(), "A test project");
    }
    // Test the mutator/setters.
    #[test]
    fn set_name_1() {
        let mut  project =Project::new(1, "Test Project", "A test project");
        project.set_name("New name");
        assert_eq!(project.name(), "New name");

    }
    #[test]
    fn set_description_1() {
        let mut project =Project::new(1, "Test Project", "A test project");
        project.set_description("A description");
        assert_eq!(project.description(), "A description");
    }
    // Test chaining

    #[test]
    fn chain_1() {
        let mut project =Project::new(1, "Test Project", "A test project");
        project.set_name("Name").set_description("Desc");

        assert_eq!(project.name(), "Name");
        assert_eq!(project.description(), "Desc");
    }
    #[test]
    fn chain_2() {
        let mut project =Project::new(1, "Test Project", "A test project");
        project.set_description("Desc").set_name("Name");


        assert_eq!(project.name(), "Name");
        assert_eq!(project.description(), "Desc");
    }

}

#[cfg(test)]
mod project_firing_step_tests {
    use super::*;

    #[test]
    fn new_1() {
        let pstep = ProjectFiringStep::new(1, 2, 3, "A step");
        assert_eq!(
            pstep,
            ProjectFiringStep {
                id : 1, project_id : 2, firing_sequence_id : 3, 
                comment : "A step".into()
            }
        );
    }
    // Selector/getters tests:
    #[test]
    fn id_1()  {
        let pstep = ProjectFiringStep::new(1, 2, 3, "A step");
        assert_eq!(pstep.id(), 1);
    }
    #[test]
    fn project_id_1() {
        let pstep = ProjectFiringStep::new(1, 2, 3, "A step");
        assert_eq!(pstep.project_id(), 2);
    }
    #[test]
    fn firing_sequence_id_1() {
        let pstep = ProjectFiringStep::new(1, 2, 3, "A step");
        assert_eq!(pstep.firing_sequence_id(), 3);
    }
    #[test]
    fn comment_1() {
        let pstep = ProjectFiringStep::new(1, 2, 3, "A step");
        assert_eq!(pstep.comment(), "A step");
    }
    // mutator/setter test.

    #[test]
    fn set_comment_1() {
        let mut pstep = ProjectFiringStep::new(1, 2, 3, "A step");
        pstep.set_comment("A new comment");
        assert_eq!(pstep.comment(), "A new comment");
    }
    #[test]
    fn set_comment_2() {
        // mutators chain:

        let mut pstep = ProjectFiringStep::new(1, 2, 3, "A step");
        pstep
            .set_comment("An intermediate comment")
            .set_comment("The final comment");
    
        assert_eq!(pstep.comment(), "The final comment");
    }
}
#[cfg(test)]
mod project_image_tests {
    use super::*;

    #[test]
    fn new_1() {
        let im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
        assert_eq!(
            im,
            ProjectImage {
                id : 1, 
                project_id : 2, 
                name : "Image.jpeg".into(),
                description: "The pieces".into(),
                contents : vec![]
            }
        );
    }
    // Test selectors/getters.

    #[test]
    fn id_1() {
        let im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
        assert_eq!(im.id(), 1);
    }
    #[test]
    fn project_id_1() {
        let im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
        assert_eq!(im.project_id(), 2);
    }
    #[test]
    fn name_1() {
        let im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
        assert_eq!(im.name(), "Image.jpeg");
    }
    #[test]
    fn description_1() {
        let im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
        assert_eq!(im.description, "The pieces");
    }
    #[test]
    fn contents_1() {
        let im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
        assert_eq!(im.contents(), vec![]);
    }
    // Test mutators.

    #[test]
    fn set_name_1() {
         let mut im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
         im.set_name("New name");
         assert_eq!(im.name(), "New name");
    }
    #[test]
    fn set_description_1() {
        let mut im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
        im.set_description("something else");
        assert_eq!(im.description(), "something else");

    }
    #[test]
    fn set_contents_1() {
        let mut im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
        let data : Vec<u8> = vec![1,2,3,4,5];
        im.set_contents(&data);
        assert_eq!(im.contents(), data);
    }

    #[test]
    fn chain_1() {
        let mut im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
        let data : Vec<u8> = vec![1,2,3,4,5];
        im.set_name("name").set_description("description").set_contents(&data);

        assert_eq!(im.name(), "name");
        assert_eq!(im.description(), "description");
        assert_eq!(im.contents(), data);
    }
    #[test]
    fn chain_2() {
        // just need to also be sure that set contents chains:
        let mut im = ProjectImage::new(1, 2, "Image.jpeg", "The pieces");
        let data : Vec<u8> = vec![1,2,3,4,5];

        im.set_contents(&data).set_name("new name");
        assert_eq!(im.contents(), data);
        assert_eq!(im.name(), "new name");
    }
}
#[cfg(test)]
mod kiln_project_tests {
    use super::*;

    #[test]
    fn new_1() {
        let project = Project::new(1, "Aproject", "test Project");
        let kp = KilnProject::new(&project);
        assert_eq!(
            kp, KilnProject {
                project : project,
                firing_comments : vec![],
                firings : vec![],
                pictures : vec![]
            }
        );
    }
    #[test]
    fn project_1() {
        let project = Project::new(1, "Aproject", "test Project");
        let kp = KilnProject::new(&project);
        assert_eq!(kp.project(), project);
    }
    //Not we'll have more tests when we get around to editing the project.
    #[test]
    fn firing_comments() {
        let project = Project::new(1, "Aproject", "test Project");
        let kp = KilnProject::new(&project);
        assert_eq!(
            kp.firing_comments(), Vec::<String>::new()
        );
    }
    #[test]
    fn firings_1() {
        let project = Project::new(1, "Aproject", "test Project");
        let kp = KilnProject::new(&project);
        assert_eq!(
            kp.firings(), Vec::<KilnProgram>::new()
        );
    }
    #[test]
    fn pictures_1(){
        let project = Project::new(1, "Aproject", "test Project");
        let kp = KilnProject::new(&project);
        assert_eq!(
            kp.pictures(), Vec::<ProjectImage>::new()
        );
    }
    #[test]
    fn num_firings_1() {
        let project = Project::new(1, "Aproject", "test Project");
        let kp = KilnProject::new(&project);
        assert_eq!(kp.num_firings(), 0);
    }
    #[test]
    fn num_imgaes() {
        let project = Project::new(1, "Aproject", "test Project");
        let kp = KilnProject::new(&project);
        assert_eq!(kp.num_images(), 0);
    }

}

