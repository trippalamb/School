use crate::significance::tokenizer::Position;

pub fn std_lib_call(name: &str, args: &Vec<f64>, pos: &Position) -> f64 {

    match name {

        "sin" => {
            if args.len() != 1 {panic!("Std lib function <sin> takes 1 argument @ {}", pos.line)};
            args[0].sin()
        },
        "cos" => {
            if args.len() != 1 {panic!("Std lib function <cos> takes 1 argument @ {}", pos.line)};
            args[0].cos()
        },
        "sqrt" => {
            if args.len() != 1 {panic!("Std lib function <sqrt> takes 1 argument @ {}", pos.line)};
            args[0].sqrt()
        },
        _ => panic!("Unknown std lib function {}", name)
    }

}