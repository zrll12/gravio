use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref POSITION_REGEX: Regex = Regex::new(r"\((?P<x>-?\d*\.?\d*)?,?\s*(?P<y>-?\d*\.?\d*)?,?\s*(?P<z>-?\d*\.?\d*)?,?\s*(?P<extra>.*)?\)").unwrap();
}

pub fn resolve_position(position: &str, index: usize) -> (String, Vec<String>) {
    if position.is_empty() {
        println!("ERROR: Position string is empty at line {}", index);
        panic!("Resolve position failed");
    }

    let captures = POSITION_REGEX.captures(position).unwrap_or_else(|| {
        println!("ERROR: Invalid position format: {} at line {}", position, index);
        println!("help: Ensure position is in the format (x, y, z, key=value, key=value)");
        panic!("Resolve position failed");
    });

    let mut result = String::new();
    let mut extra_names = Vec::new();

    if let Some(x) = captures.name("x").and_then(|m| if m.as_str().is_empty() { None } else { Some(m.as_str()) }) {
        if x.parse::<f64>().is_err() {
            println!("ERROR: Invalid character found in position tuple. found {} at line {}", x, index);
            println!("help: Ensure x is a valid number");
            panic!("Resolve position failed");
        }
        result.push_str(&format!("X{} ", x));
    }
    if let Some(y) = captures.name("y").and_then(|m| if m.as_str().is_empty() { None } else { Some(m.as_str()) }) {
        if y.parse::<f64>().is_err() {
            println!("ERROR: Invalid character found in position tuple. found {} at line {}", y, index);
            println!("help: Ensure y is a valid number");
            panic!("Resolve position failed");
        }
        result.push_str(&format!("Y{} ", y));
    }
    if let Some(z) = captures.name("z").and_then(|m| if m.as_str().is_empty() { None } else { Some(m.as_str()) }) {
        if z.parse::<f64>().is_err() {
            println!("ERROR: Invalid character found in position tuple. found {} at line {}", z, index);
            println!("help: Ensure z is a valid number");
            panic!("Resolve position failed");
        }
        result.push_str(&format!("Z{} ", z));
    }
    if let Some(extra) = captures.name("extra").and_then(|m| if m.as_str().is_empty() { None } else { Some(m.as_str()) }) {
        for param in extra.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            let parts: Vec<&str> = param.split('=').collect();
            if parts.len() == 2 {
                extra_names.push(parts[0].to_uppercase().to_string());
                result.push_str(&format!("{}{} ", parts[0].to_uppercase(), parts[1]));
            } else {
                println!("ERROR: Invalid character found in position tuple. found {} at line {}", param, index);
                println!("help: Ensure extra parameters are in the format key=value");
                panic!("Resolve position failed");
            }
        }
    }

    if result.is_empty() {
        panic!("ERROR: No valid position data found");
    } else {
        (result.trim_end().to_string(), extra_names)
    }
}

#[test]
pub fn test_position_resolver() {
    assert_eq!(resolve_position("(1.5, 1, 1)", 3), ("X1.5 Y1 Z1".to_string(), vec![]));
    assert_eq!(resolve_position("(3, , -1)", 3), ("X3 Z-1".to_string(), vec![]));
    assert_eq!(resolve_position("(2, 1, 3, I=2)", 3), ("X2 Y1 Z3 I2".to_string(), vec!["I".to_string()]));
    assert_eq!(resolve_position("(2, 1, 3.3, K=1.5)", 3), ("X2 Y1 Z3.3 K1.5".to_string(), vec!["K".to_string()]));
    assert_eq!(resolve_position("(30, -2, , K=90)", 3), ("X30 Y-2 K90".to_string(), vec!["K".to_string()]));
    assert_eq!(resolve_position("(30, -2, , R=20)", 3), ("X30 Y-2 R20".to_string(), vec!["R".to_string()]));
    assert_eq!(resolve_position("(30, -2, I=45, J=90)", 3), ("X30 Y-2 I45 J90".to_string(), vec!["I".to_string(), "J".to_string()]));
    assert_eq!(resolve_position("(30, -2, , p=10, Q=20)", 3), ("X30 Y-2 P10 Q20".to_string(), vec!["P".to_string(), "Q".to_string()]));
}