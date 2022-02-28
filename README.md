# compiler-dev

北大编译实践教学用编译器开发环境 (Compiler Development Environment).

该仓库的内容将被打包为 Docker 镜像, 所以不建议直接使用该仓库, 具体使用方法见[使用方法](#使用方法)一节.

## 使用方法

### 从 Docker Hub 获取

推荐使用该方法.

```sh
docker run -it --rm maxxing/compiler-dev bash
```

### 从仓库构建

```sh
cd docker
make # or `sudo make`
docker run -it --rm compiler-dev bash
```

## 镜像中包含的内容

* 必要的工具: `git`, `flex`, `bison`, `python3` (用于运行测试脚本).
* 构建工具: `make`, `cmake`.
* 运行工具: `qemu-user-static`.
* 编译工具链: Rust 工具链, LLVM 工具链.
* Koopa IR 相关工具: `libkoopa`, `koopac`.
* 测试脚本: `autotest`.

## 缺陷/待处理

* [ ] 目前的镜像会安装各类预编译工具链 (例如 LLVM), 但其中部分工具链不支持 `aarch64`, 这也许会给使用搭载了 Apple Silicon 平台的同学带来困扰. 是否考虑从源代码编译?

## 变更日志

见 [CHANGELOG.md](CHANGELOG.md).
