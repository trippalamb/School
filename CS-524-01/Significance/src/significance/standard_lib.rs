use crate::significance::tokenizer::Position;
use crate::{Number, Real};

pub fn std_lib_call(name: &str, args: &Vec<Real>, pos: &Position) -> Real {

    //TODO: add uncertainty calculation for std lib functions
    match name {

        "sin" => {
            if args.len() != 1 {panic!("Std lib function <sin> takes 1 argument @ {}", pos.line)};
            Real::with_error(args[0].value().sin(), 0.0)
        },
        "cos" => {
            if args.len() != 1 {panic!("Std lib function <cos> takes 1 argument @ {}", pos.line)};
            Real::with_error(args[0].value().cos(), 0.0)
        },
        "sqrt" => {
            if args.len() != 1 {panic!("Std lib function <sqrt> takes 1 argument @ {}", pos.line)};
            Real::with_error(args[0].value().sqrt(), 0.0)
        },
        _ => panic!("Unknown std lib function {}", name)
    }

}