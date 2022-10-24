
/*
    以下のコマンドを実行してコンパイルするとエラーが発生しない。
    rustup target add thumbv7em-none-eabihf
    cargo build --target thumbv7em-none-eabihf
*/

#![no_std] // Rust の標準ライブラリにリンクしない

#![no_main] // 全ての Rust レベルのエントリポイントを無効にする
// fn main(){}

use core::panic::PanicInfo;

#[no_mangle] // この関数の名前修飾をしない
pub extern "C" fn _start() -> ! {
    // リンカはデフォルトで `_start` という名前の関数を探すので、
    // この関数がエントリポイントとなる
    loop {}
}

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}