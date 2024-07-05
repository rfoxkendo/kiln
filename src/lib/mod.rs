
//! Provides the structs and enumerated types that define internal representations of Kiln programs.
//!   *  Rates are ramp rates in either degrees/hr or AFAP for "As Fast As Possible".
//!   *  Steps represent a program step.  They have a target temp, a rate and a hold time.
//!   *  Programs are just a named, and commented vector of steps.
//!    * A Run is a program, a date/time and a textual command with an optional image (which is just a binary vector(?))
//! and image type the purpose of the image is to  show how the run worked out (the finished result of the kiln run).
//! NOTE: - in tyhe future a run may have a vector of images.
#![crate_name="programs"]
pub mod programs {
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
    pub fn new(target: f32, ramp : RampRate, hold: u32) -> Step {
        Step {
            target : target, ramp_rate: ramp, hold_time: hold
        }
    }
    pub fn target_temp(self) -> f32 {
        self.target
    }
    pub fn ramp_rate(self) -> RampRate {
        self.ramp_rate
    }
    pub fn hold_time(self) -> u32 {
        self.hold_time
    }
}

/// A fully described kil program:
/// 
#[derive(Clone, Debug)]
pub struct Program {
    name : String,
    description : String,
    program : Vec<Step>
}

/// A project is a description, a time/date that it was run
/// A second string describing how happy we ware with it.
/// and more to be added later (vector of images).
/// 
#[derive(Clone, Debug)]
pub struct Project {
    description : String,
    result : String,
    program : Program,
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


}

