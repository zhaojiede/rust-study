/* 错误处理 */
/* Rust 没有异常。相反，它有 Result<T, E> 类型，用于处理可恢复的错误，
    还有 panic! 宏，在程序遇到不可恢复的错误时停止执行。 */

/* 用 panic! 处理不可恢复的错误 */
/* 两种方式造成 panic: 执行会造成 panic 的操作（比如访问超过数组结尾的内容）或者显示调用 panic! 宏。
    通常情况下这些 panic 会打印一个错误信息，展开并清理栈数据，然后退出。
    通过一个环境变量，你也可以让 Rust 在 panic 发生时打印调用堆栈（call stack) 以便于定位 panic 的原因。 */
fn main() {
    // 手动调用 panic!
    // panic!("crash and burn");

    out_arrary_index_error();
}

// 会报错：
// thread 'main' panicked at src/main.rs:18:6:              // panic 发生的位置
// index out of bounds: the len is 3 but the index is 99            // panic 的原因
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace        // 提示我们可以设置 RUST_BACKTRACE
                                                                                        // 环境变量来得到一个 backtrace
// backtrace: 一个执行到目前位置所有被调用函数列表。
// 使用 backtrace 需要设置 RUST_BACKTRACE=1 环境变量。
// linux or mac: RUST_BACKTRACE=1 cargo run
// win powershell: $env:RUST_BACKTRACE=1; cargo run

fn out_arrary_index_error() {
    let v = vec![1, 2, 3];
    v[99];
}
