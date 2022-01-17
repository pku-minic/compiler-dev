# koopac

`koopac`: Koopa IR to LLVM IR converter.

Koopa IR 到 LLVM IR 的转换器, 用于支持 Koopa IR 程序的运行.

## 用法

命令行: `koopac [INPUT] [-o OUTPUT] [-h|--help]`.

选项:

* `INPUT`: 输入的 Koopa IR 文件, 默认为标准输入 (`stdin`).
* `OUTPUT`: 输出的  LLVM IR 文件, 默认为标准输出 (`stdout`).
* `-h` 或 `--help`: 查看帮助信息.
