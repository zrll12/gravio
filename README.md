<div align="center">
    <h1> Gravio </h1>
</div>

<p align="center">
    <b>
	English | 
        <a href="./README_CN.md">简体中文</a>
    </b>
</p>

Gravio (GRO) is a language that employs RISC (Reduced Instruction Set Computer) style assembly syntax, allowing for easy code writing that can be compiled into G-code, thereby simplifying the CNC (Computer Numerical Control) programming process.

## Example

### G-code Input

```gcode
N10 G54 G90 G94 G23;
N20 T1 M03;
N30 G00 Z2;
N40 G00 X35;
N50 G01 X26 Z0 F100;
N60 G03 X30 Z-2 CR=2;
N70 G01 X30 Z-20;
N80 G01 X40 Z-25;
N90 G01 X40 Z-35;
N100 G01 X50;
N110 G00 Z50;
N120 M05;
N130 M02;
```

### Compiled Gravio Output

```gravio
ORIGINAL 0;
ABSOLUTE;
LINER;
DIAMETER;
TOOL 1;
MOTOR start_clockwise;

JUMP (,,2);
JUMP (35,,);
GLIDE (26,,0,F=100);
TURN counter_clockwise (30,,-2,I=2);
GLIDE (30,,-20);
GLIDE (40,,-25);
GLIDE (40,,-35);
GLIDE (50,,);
JUMP (,,50);

MOTOR stop;
MOTOR return;
```

## Features

- **User-friendly**: Intuitive assembly-style syntax that is easy to read.
- **Error handling**：Provides detailed error messages.

## Usage

To use Gravio, you need to set up a Rust environment. Please follow these steps:

- Clone the repository.
- Ensure Rust and Cargo are installed.
- Run `cargo clippy` to check the code.
- Use `cargo build` to compile the project.
- Use the provided parser to convert Gravio code into G-code.

## TODO

 - [ ] Dialect parsing: Introduce rules for specific dialects to enhance adaptability.
 - [ ] Optimize interpreter logic: Simplify the interpreter for improved performance.
 - [ ] Unit testing: Ensure stability and correctness.
