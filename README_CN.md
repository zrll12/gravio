<div align="center">
    <h1> Gravio </h1>
</div>

<p align="center">
    <b>
        <a href="./README.md">English</a>
	 | 简体中文
    </b>
</p>

Gravio (GRO) 是一种采用 RISC（精简指令集计算机）风格汇编语法的语言，能够方便地编写代码并将其编译为 G-code，从而简化 CNC (计算机数控) 编程过程。

## 示例

### G-code 输入

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

### 编译后的 Gravio 输出

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

## 特性

- **简单易用**：直观的汇编风格语法，便于阅读。
- **错误处理**：提供详细的错误信息。

## 使用方法

要使用 Gravio，您需要设置 Rust 环境。请按照以下步骤操作：

- 克隆仓库。
- 确保已安装 Rust 和 Cargo。
- 运行 `cargo clippy` 进行代码检查。
- 使用 `cargo build` 构建项目。
- 使用提供的解析器将 Gravio 代码转换为 G-code。

## TODO

 - [ ] 方言解析：引入特定方言的规则以增强适应性。
 - [ ] 优化解释器逻辑：简化解释器以提高性能。
 - [ ] 单元测试：确保稳定性和正确性。
