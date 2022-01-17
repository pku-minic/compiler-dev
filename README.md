# compiler-dev

北大编译实践教学用编译器开发环境 (Compiler Development Environment).

该仓库的内容将被打包为 Docker 镜像, 所以不建议直接使用该仓库, 具体使用方法见[使用方法](#使用方法)一节.

## 使用方法

> WIP.

## 镜像中包含的内容

* 必要的工具: `git`, `flex`, `bison`, `python3` (用于运行测试脚本).
* 构建工具: `make`, `cmake`.
* 运行工具: `qemu-user-static`.
* 编译工具链: Rust 工具链, LLVM 工具链, RISC-V 工具链.
* Koopa IR 相关工具: `libkoopa`, `koopac`.
* 测试脚本.
