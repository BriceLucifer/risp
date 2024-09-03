# Risp

欢迎使用 Risp，这是一个用 Rust 编写的简单 Lisp REPL（读取-求值-打印循环）。Risp 提供了一个轻量级的环境，让您可以快速地执行 Lisp 表达式，并查看结果。

## 功能

- 支持基本的 Lisp 表达式求值
- 支持整数、布尔值、符号和 Lambda 表达式的输出
- 颜色编码输出，以便于区分不同类型的结果
- 使用 `exit` 命令可以优雅地退出 REPL
- 自定义提示符，增强用户体验

## 快速开始

要开始使用 Risp，您需要在系统上安装 Rust。您可以从 [官方网站](https://www.rust-lang.org/) 下载 Rust。

安装 Rust 后，您可以按照以下步骤构建并运行 Risp：

```bash
git clone https://github.com/BriceLucifer/risp.git
cd risp/src
cargo build --release
cargo run --release
```

## 使用方法

在 Risp REPL 中，您可以输入 Lisp 表达式，解释器将对其求值并以不同颜色打印结果。例如：

```lisp
🦀lisp-rs >> (+ 1 2 3)
6
🦀lisp-rs >> (if (> 10 5) 2 4)
2
🦀lisp-rs >> (define square (lambda (x) (* x x)))
🦀lisp-rs >> (square 4)
16
🦀lisp-rs >> (define fib (lambda (n) (if (< n 2) 1 (+ (fib (- n 1)) (fib (- n 2))))))
🦀lisp-rs >> (fib 10 )
89
```

要退出 Risp REPL，请输入 `exit`。

## 贡献

欢迎为 Risp 做出贡献！如果您有改进的想法或发现了错误，请开一个 issue 或提交一个 pull request。

## 许可证

Risp 在 MIT 许可证下获得许可。有关更多细节，请查看 [LICENSE](LICENSE) 文件。

---

祝您使用 Risp 愉快！
