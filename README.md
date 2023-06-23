# TinyRust

用 Rust 写一个 Tiny Rust 编译器。

但事实上，它可能不包含 Rust 任何复杂的特性，如生命周期、borrow check、FFI 等。它只支持一些简单的运算，任何语言都包含这些特性，因此它可以叫 Tiny C，Tiny Java，Tiny Go，只是它的语法跟 Rust 一致罢了。其次，它的实现可能也与 Rust Compiler 或任何编译器教材中都天差地别，

## 目标

当我用 Cargo 新建一个项目时，Cargo 生成了一个 `main.rs` 的文件，内容为：

```Rust
fn main() {
    println!("Hello, world!");
}
```

Tiny Rust的目标，就是能够编译这段代码。我将其保存在根目录下，并命名为 `hello_world.trs`。
