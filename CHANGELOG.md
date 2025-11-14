# 变更日志

关于 `compiler-dev` Docker 镜像的所有变更将陈列至此.

## 未发布

### 变更

* [PR #3](https://github.com/pku-minic/compiler-dev/pull/3): enabled Ubuntu-Ports mirror for arm64, armhf, PowerPC, ppc64el, s390x.
* [PR #5](https://github.com/pku-minic/compiler-dev/pull/5): Change tuna mirror to llvm.org source.
* 将 `libkoopa` 和 `koopac` 的 `koopa` 依赖更新到 0.0.9 版本, 并支持 Rust 2024.
* 将 `compiler-dev` 的基础镜像更新到 `ubuntu:24.04`.

## 0.0.8 - 2023-06-02

### 变更

* 将 `libkoopa` 和 `koopac` 的 `koopa` 依赖更新到 0.0.7 版本.
* 将 Rust 工具链更新至 1.70.0, 同时启用 sparse registry.

## 0.0.7 - 2023-01-12

### 变更

* 将 `libkoopa` 和 `koopac` 的 `koopa` 依赖更新到 0.0.6 版本.

### 修复

* `koopac` 代码中的一些过时的写法.

## 0.0.6 - 2022-05-07

### 变更

* 目前的 `autotest` 在执行 `make` 构建时会指定 `DEBUG=0`.

### 修复

* `autotest` 中关于提示信息的一些笔误.

## 0.0.5 - 2022-03-14

### 新增

* 目前的 `koopac` 会将输入 Koopa IR 中所有的 `alloc` 指令提升至入口基本块.

## 0.0.4 - 2022-03-10

### 变更

* 将 `libkoopa` 和 `koopac` 的 `koopa` 依赖更新到 0.0.5 版本.

### 修复

* `autotest` 脚本中关于 `Popen` 相关操作的问题.

## 0.0.3 - 2022-03-05

### 变更

* 将测试用例 `multiple_returns` 从 `lv4` 目录移到了 `lv6` 目录.

## 0.0.2 - 2022-02-28

### 修复

* 测试用例 (`compiler-dev-test-cases`) 中存在的部分问题.

## 0.0.1 - 2022-02-22
