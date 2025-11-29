//------------------
// (Tripp) Milton Lamb
// Fall 2025, Nov 29 2025
// CS-524: Programming Languages
// Final Project
//------------------

use crate::significance::tokenizer::Position;
use crate::{Number, Real};

pub fn std_lib_call(name: &str, args: &Vec<Real>, pos: &Position) -> Real {

    match name {

        "sin" => {
            if args.len() != 1 {panic!("Std lib function <sin> takes 1 argument @ {}", pos.line)};
            let error = args[0].value().cos().abs() * args[0].error();
            Real::with_error(args[0].value().sin(), error)
        },
        "cos" => {
            if args.len() != 1 {panic!("Std lib function <cos> takes 1 argument @ {}", pos.line)};
            let error = args[0].value().sin().abs() * args[0].error();
            Real::with_error(args[0].value().cos(), error)
        },
        "sqrt" => {
            if args.len() != 1 {panic!("Std lib function <sqrt> takes 1 argument @ {}", pos.line)};
            args[0].root(Real::new(2.0))
        },
        _ => panic!("Unknown std lib function {}", name)
    }

}