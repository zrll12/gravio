mod position;

pub fn resolve(params: Vec<&str>, index: usize) -> Option<(String, u8)> {
    if params.is_empty() || params[0].is_empty() {
        return None;
    }
    let mut warning = 0;

    let code = match params[0] {
        "JUMP" => {
            if params.len() < 2 {
                println!("ERROR: Expected to have 1 params, found {} at line {}", params.len() - 1, index);
                println!("help: Usage: JUMP {{position: Position}}");
                panic!("Missing params");
            }
            if params.len() != 2 {
                warning += 1;
                println!("WARN: Command JUMP has {} params, expected 2 at line {}", params.len(), index);
            }

            let (result, _) = position::resolve_position(params[1], index);

            Some(format!("G0 {}", result))
        }
        "GLIDE" => {
            if params.len() < 2 {
                println!("ERROR: Expected to have 1 params, found {} at line {}", params.len() - 1, index);
                println!("help: Usage: GLIDE {{position: Position}}");
                panic!("Missing params");
            }
            if params.len() != 2 {
                warning += 1;
                println!("WARN: Command GLIDE has {} params, expected 2 at line {}", params.len(), index);
            }

            let (result, _) = position::resolve_position(params[1], index);

            Some(format!("G1 {}", result))
        }
        "TURN" => {
            if params.len() < 3 {
                println!("ERROR: Expected to have 2 params, found {} at line {}", params.len() - 1, index);
                println!("help: Usage: TURN {{direction: Direction}} {{position: Position}}");
                println!("      Direction: 'clockwise'('cw'), 'counter_clockwise'('ccw')");
                panic!("Missing params");
            }
            if params.len() != 3 {
                warning += 1;
                println!("WARN: Command TURN has {} params, expected 2 at line {}", params.len(), index);
            }

            let (result, _) = position::resolve_position(params[2], index);
            let direction = match params[1] {
                "clockwise" | "cw" => true,
                "counter_clockwise" | "ccw" => false,
                _ => {
                    println!("ERROR: Unknown direction {} at line {}", params[1], index);
                    panic!("Resolve direction failed")
                }
            };

            if direction {
                Some(format!("G2 {}", result))
            } else {
                Some(format!("G3 {}", result))
            }
        }
        "ORIGINAL" => {
            if params.len() < 2 {
                println!("ERROR: Expected to have 1 params, found {} at line {}", params.len() - 1, index);
                println!("help: Usage: TOOL {{type: number}}");
                panic!("Missing params");
            }
            if params.len() != 2 {
                warning += 1;
                println!("WARN: Command LINER has {} params, expected 2 at line {}", params.len(), index);
            }
            let Ok(index) = params[1].parse::<u8>() else {
                println!("ERROR: Mismatched type. Need number, found {} at line {}", params[1], index);
                panic!("Mismatched type");
            };
            if index > 3 {
                println!("ERROR: Expected type index to be in range 0-3, found {} at line {}", params[1], index);
                panic!("Mismatched type");
            }

            Some(format!("G{}", 54 + index))
        }
        "DIAMETER" => {
            if params.len() > 1 {
                warning += 1;
                println!("WARN: Command ABSOLUTE has {} params, expected 1 at line {}", params.len(), index);
            }
            Some("G23".to_string())
        }
        "ABSOLUTE" => {
            if params.len() > 1 {
                warning += 1;
                println!("WARN: Command ABSOLUTE has {} params, expected 1 at line {}", params.len(), index);
            }
            Some("G90".to_string())
        }
        "LINER" => {
            if params.len() > 1 {
                warning += 1;
                println!("WARN: Command LINER has {} params, expected 1 at line {}", params.len(), index);
            }
            Some("G94".to_string())
        }
        "TOOL" => {
            if params.len() < 2 {
                println!("ERROR: Expected to have 1 params, found {} at line {}", params.len() - 1, index);
                println!("help: Usage: TOOL {{type: number}}");
                panic!("Missing params");
            }
            if params.len() != 2 {
                warning += 1;
                println!("WARN: Command LINER has {} params, expected 2 at line {}", params.len(), index);
            }
            if !is_number(params[1]) {
                println!("ERROR: Mismatched type. Need number, found {} at line {}", params[1], index);
                panic!("Mismatched type");
            }

            Some(format!("T{}", params[1]))
        }
        "MOTOR" => {
            if params.len() < 2 {
                println!("ERROR: Expected to have 1 params, found {} at line {}", params.len() - 1, index);
                println!("help: Usage: MOTOR {{motion: MotionType | number}}");
                println!("      MotionType: 'force_stop', 'stop', 'start_clockwise'...");
                panic!("Missing params");
            }
            if params.len() != 2 {
                warning += 1;
                println!("WARN: Command LINER has {} params, expected 2 at line {}", params.len(), index);
            }

            let operation: &str = match params[1] {
                "force_stop" => {"1"}
                "return" => {"2"}
                "start_clockwise" => {"3"}
                "strat_counter_clockwise" => {"4"}
                "stop" => {"5"}
                _ => {
                    if params[1].parse::<u32>().is_ok() {
                        params[1]
                    } else {
                        println!("ERROR: Unknown type {} at line {}", params[1], index);
                        panic!("Mismatched type")
                    }
                }
            };

            Some(format!("M{operation}"))
        }
        _ => {
            warning += 1;
            println!("WARN: Command not recognised: {} at line {}", params[0], index);
            None
        }
    };

    code.map(|c| (c, warning))
}

fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}