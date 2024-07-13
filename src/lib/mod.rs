
//! Provides the structs and enumerated types that define internal representations of Kiln programs.
//!   *  Rates are ramp rates in either degrees/hr or AFAP for "As Fast As Possible".
//!   *  Steps represent a program step.  They have a target temp, a rate and a hold time.
//!   *  Programs are just a named, and commented vector of steps.
//!    * A Run is a program, a date/time and a textual command with an optional image (which is just a binary vector(?))
//! and image type the purpose of the image is to  show how the run worked out (the finished result of the kiln run).
//! NOTE: - in tyhe future a run may have a vector of images.
#![crate_name="programs"]
pub mod programs {
    use chrono::prelude::*;
    /// How fast the kiln should go from its current temperature to the next one.
    /// 
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum RampRate {
        AFAP,
        DegreesPerHour(f32)
    }

    /// A step in a kiln program:
    ///
    #[derive(Copy, Clone, PartialEq, Debug)] 
    pub struct Step {
        target : f32,
        ramp_rate : RampRate,
        hold_time : u32
    }

    impl Step {
        /// Create a new step object.
        pub fn new(target: f32, ramp : RampRate, hold: u32) -> Step {
            Step {
                target : target, ramp_rate: ramp, hold_time: hold
            }
        }
        /// Selector - return the target tempaerature of a step.
        pub fn target_temp(self) -> f32 {
            self.target
        }
        /// selector - return the ramp rate for a step.
        pub fn ramp_rate(self) -> RampRate {
            self.ramp_rate
        }
        /// Selector - return the hold time for a step.
        pub fn hold_time(self) -> u32 {
            self.hold_time
        }
    }

    /// A fully described kiln program:
    /// 
    #[derive(Clone, Debug, PartialEq)]
    pub struct Program {
        name : String,
        description : String,
        program : Vec<Step>
    }
    impl Program {
        ///Create a program with no steps.
        pub fn new (name : &str, description : &str) -> Program {
            Program {
                name : String::from(name),
                description: String::from(description),
                program : vec![]
            }
        }
        /// Create a program with an initial set of steps.
        pub fn from_steps(name : &str, description : &str, program: &Vec<Step>) -> Program {
            Program {
                name : String::from(name),
                description : String::from(description),
                program : program.clone()
            }
        }
        /// Append a new step to a program.
        pub fn add_step(&mut self, step : Step) -> &Program {
            self.program.push(step);
            self
        }
        /// Append a bunch of new steps to a program.  e.g. take an empty program
        /// and define its steps.
        pub fn add_steps(&mut self, steps : &Vec<Step>) -> &Program {
            self.program.extend_from_slice(steps.as_slice());

            self
        }
        /// Clear the steps from a program making it empty.
        pub fn clear(&mut self) -> &Program {
            self.program.clear();
            self
        }
        /// Selector - return a clone of the steps.
        pub fn steps(&self) -> Vec<Step> {
            self.program.clone()
        }
        /// Selector - return the name of a program.
        pub fn name(&self) -> String {
            self.name.clone()
        }
        /// Selector - Return  a program's description.
        pub fn description(&self) -> String {
            self.description.clone()
        }
    }

    /// A project is a description, a time/date that it was run
    /// A second string describing how happy we ware with it.
    /// and more to be added later (vector of images).
    /// 
    #[derive(Clone, Debug)]
    pub struct Project {
        run_at : DateTime<Utc>,
        description : String,
        result : String,
        program : Program,
    }
    impl Project {
    
    }


    #[cfg(test)]
    mod step_tests {
        use super::*;

        #[test]
        fn new_0() {
            let r = Step::new(1000.0, RampRate::DegreesPerHour(100.0), 32);
            assert_eq!(
                r, Step {target: 1000.0, ramp_rate: RampRate::DegreesPerHour(100.0), hold_time: 32}
            );
        }
        #[test]
        fn target_0() {
            let r = Step::new(1000.0, RampRate::DegreesPerHour(100.0), 32);
            assert_eq!(r.target_temp(), 1000.0);
        }
        #[test]
        fn ramp_0() {
            let r = Step::new(1000.0, RampRate::DegreesPerHour(100.0), 32);
            assert_eq!(r.ramp_rate(), RampRate::DegreesPerHour(100.0));

        }
        #[test]
        fn ramp_1() {
            let r = Step::new(1000.0, RampRate::AFAP, 32);
            assert_eq!(r.ramp_rate(), RampRate::AFAP);
        }
        #[test]
        fn hold_1() {
            let r = Step::new(1000.0, RampRate::DegreesPerHour(100.0), 32);
            assert_eq!(r.hold_time(), 32);
        }
    }

    #[cfg(test)]
    mod program_tests {
        use super::*;
        #[test]
        fn new_0()  {
            // Empty program.
            let pgm = Program::new("small-full", "Full fuse for small pieces");
            assert_eq!(
                pgm, Program {
                    name : String::from("small-full"),
                    description: String::from("Full fuse for small pieces"),
                    program : vec![]
                }
            );
        }
        #[test]
        fn new_1() {
            // With a program.

            let steps = vec![
                Step::new(1000.0, RampRate::DegreesPerHour(300.0), 30),
                Step::new(1250.0, RampRate::DegreesPerHour(300.0), 15),
                Step::new(1450.0, RampRate::DegreesPerHour(500.0), 15),
                Step::new(900.0, RampRate::AFAP, 30)
            ];
            let pgm = Program::from_steps("small-full", "Full fuse for small pieces", &steps);
            assert_eq!(
                pgm, Program {
                    name : String::from("small-full"),
                    description: String::from("Full fuse for small pieces"),
                    program : steps.clone()
                }
            );
        }
        #[test]
        fn add_1() {
            // Add  a step to an empty project:

            let mut pgm = Program::new("testing", "Test project");
            assert_eq!(*pgm.add_step(Step::new(1000.0, RampRate::DegreesPerHour(300.0), 30)),
                Program {
                    name : String::from("testing"),
                    description : String::from("Test project"),
                    program : vec![Step::new(1000.0, RampRate::DegreesPerHour(300.0), 30)]
                }
            );

        }
        #[test]
        fn add_2() {
            // add a step to a nonempty program:

            let mut steps = vec![
                Step::new(1000.0, RampRate::DegreesPerHour(300.0), 30),
                Step::new(1250.0, RampRate::DegreesPerHour(300.0), 15),
                Step::new(1450.0, RampRate::DegreesPerHour(500.0), 15),
                Step::new(900.0, RampRate::AFAP, 30)
            ];
            let mut pgm = Program::from_steps("small-full", "Full fuse for small pieces", &steps);
            let next_step = Step::new(80.0, RampRate::AFAP, 100);
            steps.push(next_step.clone());
            assert_eq!(
                *pgm.add_step(next_step),
                Program {
                    name : String::from("small-full"),
                    description : String::from("Full fuse for small pieces"),
                    program : steps
                }
            );
        }
        #[test]
        fn add_3() {
            // add multiple steps to an empty project:

            let mut pgm = Program::new("small-full", "Full fuse for small pieces");
            let steps = vec![
                Step::new(1000.0, RampRate::DegreesPerHour(300.0), 30),
                Step::new(1250.0, RampRate::DegreesPerHour(300.0), 15),
                Step::new(1450.0, RampRate::DegreesPerHour(500.0), 15),
                Step::new(900.0, RampRate::AFAP, 30)
            ];
            assert_eq!(
                *pgm.add_steps(&steps),
                Program {
                    name : String::from("small-full"),
                    description: String::from("Full fuse for small pieces"),
                    program: steps
                }
            );
        }
        #[test]
        fn add_4() {
            // Add multiple steps on non-empty project:

            let mut steps = vec![
                Step::new(1000.0, RampRate::DegreesPerHour(300.0), 30),
                Step::new(1250.0, RampRate::DegreesPerHour(300.0), 15),
            
            ];
            let more_steps = vec![
                Step::new(1450.0, RampRate::DegreesPerHour(500.0), 15),
                Step::new(900.0, RampRate::AFAP, 30)
            ];

            let mut pgm = Program::from_steps("testing", "description", &steps);
            steps.extend_from_slice(more_steps.as_slice());
            assert_eq!(
                *pgm.add_steps(&more_steps),
                Program {
                    name : String::from ("testing"), description : String::from("description"),
                    program : steps
                }
            );   
        }
        #[test]
        fn clear_0() {
            // Clear empties the program steps:
            let steps = vec![
                Step::new(1000.0, RampRate::DegreesPerHour(300.0), 30),
                Step::new(1250.0, RampRate::DegreesPerHour(300.0), 15),
                Step::new(1450.0, RampRate::DegreesPerHour(500.0), 15),
                Step::new(900.0, RampRate::AFAP, 30)
            ];
            let mut pgm = Program::from_steps("small-full", "Full fuse for small pieces", &steps);
            assert_eq!(
                *pgm.clear(),
                Program {
                    name : String::from("small-full"),
                    description: String::from("Full fuse for small pieces"),
                    program: vec![]
                }
            );
        }
        #[test]
        fn selector_name() {
            let steps = vec![
                Step::new(1000.0, RampRate::DegreesPerHour(300.0), 30),
                Step::new(1250.0, RampRate::DegreesPerHour(300.0), 15),
                Step::new(1450.0, RampRate::DegreesPerHour(500.0), 15),
                Step::new(900.0, RampRate::AFAP, 30)
            ];
            let pgm = Program::from_steps("small-full", "Full fuse for small pieces", &steps);
            assert_eq!(pgm.name(), String::from ("small-full"));
        }
        #[test]
        fn selector_description() {
            let steps = vec![
                Step::new(1000.0, RampRate::DegreesPerHour(300.0), 30),
                Step::new(1250.0, RampRate::DegreesPerHour(300.0), 15),
                Step::new(1450.0, RampRate::DegreesPerHour(500.0), 15),
                Step::new(900.0, RampRate::AFAP, 30)
            ];
            let pgm = Program::from_steps("small-full", "Full fuse for small pieces", &steps);
            assert_eq!(pgm.description(), String::from("Full fuse for small pieces"));
        }
        #[test]
        fn selector_steps() {
            let steps = vec![
                Step::new(1000.0, RampRate::DegreesPerHour(300.0), 30),
                Step::new(1250.0, RampRate::DegreesPerHour(300.0), 15),
                Step::new(1450.0, RampRate::DegreesPerHour(500.0), 15),
                Step::new(900.0, RampRate::AFAP, 30)
            ];
            let pgm = Program::from_steps("small-full", "Full fuse for small pieces", &steps);
            assert_eq!(pgm.steps(), steps);
        }
    }


}

