use std::{fs::{self, OpenOptions}, io::{BufRead, BufReader, Write}};

mod resolver;

fn main() -> Result<(), String> {
    let path = "examples/basic_column.gro";
    let file = fs::File::open(path)
        .map_err(|e| format!("Cannot open file {}: {}", path, e))?;

    let file = BufReader::new(file);
    let mut gcode = Vec::new();
    let mut warnings = 0;

    println!("Compiling file: {}", path);
    for (index, line) in file.lines().enumerate() {
        let line = line.expect("Cannot read line");
        let part: Vec<&str> = line.split(";").collect();
        let params: Vec<&str> = part[0].split(" ").collect();

        let Some((result, warning)) = resolver::resolve(params, index + 1) else { continue; };
        warnings += warning;
        gcode.push(result);
    }

    let mut output = String::new();
    for (index, code) in gcode.iter().enumerate() {
        let index = index + 1;
        output.push_str(format!("N{index} {code};\n").as_str());
    }

    let mut output_file = OpenOptions::new()
        .truncate(false)
        .write(true)
        .open("output.gcode").unwrap();
    output_file.write_all(output.as_bytes()).unwrap();

    println!("Compile succeeded. lines: {} warnings: {}", gcode.len(), warnings);

    Ok(())
}