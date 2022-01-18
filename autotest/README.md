# autotest

编译器自动测试脚本.

## 用法

```sh
./autotest [-koopa|-riscv|-perf] [-t TEST_CASE_DIR] REPO_DIR
```

参数:

* `-koopa|-riscv|-perf`: 可选, 指定对编译器进行何种测试, 可为 Koopa IR 测试 (`-koopa`),  RISC-V 测试 (`-riscv`) 和性能测试 (`-perf`). 默认为 `-koopa`.
* `-t TEST_CASE_DIR`: 可选, 指定测试用例目录, 目录内须包含若干 `.c`/`.sy` 文件 (也可位于子目录中). 其中, 每个 `.c`/`.sy` 文件应当有一个位于相同目录中且同名的 `.out` 文件, 对应该 SysY 程序的预期输出/返回值; 每个 `.c`/`.sy` 文件还可以有一个位于相同目录中且同名的 `.in` 文件, 对应该 SysY 程序的输入, 如果程序不会从 `stdin` 读取输入, 则该文件应该省略. (参考[开放测试用例](https://github.com/pku-minic/open-test-cases)的写法). 默认为脚本目录下的 `testcases` 目录.
* `REPO_DIR`: 必选, 指定编译器仓库的目录. 目录内须包含 `Makefile`/`CMakeLists.txt`/`Cargo.toml` 文件之一, 脚本将自动使用 `make`/`CMake`/`Cargo` 构建编译器, 并对其进行测试.

## 对编译器的要求

* **Koopa IR 测试:** 编译器需支持形如 `./compiler -koopa INPUT -o OUTPUT` 的命令行参数, 其中 `INPUT` 为 SysY 的输入文件, `OUTPUT` 为 Koopa IR 的输出文件.
* **RISC-V 测试:** 编译器需支持形如 `./compiler -riscv INPUT -o OUTPUT` 的命令行参数, 其中 `INPUT` 为 SysY 的输入文件, `OUTPUT` 为 RISC-V 的输出文件.
* **性能测试:** 编译器需支持形如 `./compiler -perf INPUT -o OUTPUT` 的命令行参数, 其中 `INPUT` 为 SysY 的输入文件, `OUTPUT` 为 RISC-V 的输出文件.
