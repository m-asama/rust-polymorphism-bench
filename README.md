# Rust でトレイトオブジェクトと Enum でポリモーフィズムを実装した時の速度比較

なんとなくポリモーフィズムっぽく書きたいことがあったんだけどトレイトオブジェクトの動的ディスパッチってどれくらいコストが高いんだろう？と気になったので試してみた。

ポリモーフィズムする実体はたかだか数個くらいを前提。

トレイトオブジェクトの方はこんな感じでトレイト `DynObj` を実装した構造体を 5 つ用意して `Vec<Box<dyn DynObj>>` にランダムな 1,000,000 個を作って `ret` を実行。

```rust
pub trait DynObj {
    fn ret(&self) -> usize;
}

struct DynObj1 {}

impl DynObj for DynObj1 {
    fn ret(&self) -> usize {
        1
    }
}

struct DynObj2 {}

... (省略) ...

impl DynObj for DynObj5 {
    fn ret(&self) -> usize {
        5
    }
}

... (省略) ...

pub fn run(dynobjs: &Vec<Box<dyn DynObj>>) -> usize {
    let mut x = 0;
    for dynobj in dynobjs {
        x += dynobj.ret();
    }
    x
}
```

Enum の方はこんな感じで 5 つの構造体と Enum を用意して `Vec<EnmObj>` にランダムな 1,000,000 個を作って `ret` を実行。

```rust
pub struct EnmObj1 {}

impl EnmObj1 {
    fn ret(&self) -> usize {
        1
    }
}

pub struct EnmObj2 {}

... (省略) ...

impl EnmObj5 {
    fn ret(&self) -> usize {
        5
    }
}

pub enum EnmObj {
    EnmObj1(EnmObj1),
    EnmObj2(EnmObj2),
    EnmObj3(EnmObj3),
    EnmObj4(EnmObj4),
    EnmObj5(EnmObj5),
}

... (省略) ...

pub fn run(enmobjs: &Vec<EnmObj>) -> usize {
    let mut x = 0;
    for enmobj in enmobjs {
        x += enmobj.ret();
    }
    x
}
```

`main` の方はこんな感じ。

```rust
mod dynobj;
mod enmobj;

use std::time::SystemTime;

use rand::Rng;

fn main() { 
    let mut rng = rand::thread_rng();
    let mut rands = vec![];
    for _ in 0..1000000 {
        rands.push(rng.gen_range(1..=5));
    }
    let dynobjs = dynobj::build(&rands);
    let enmobjs = enmobj::build(&rands);

    let beg = SystemTime::now();
    let ret = dynobj::run(&dynobjs);
    let end = SystemTime::now();
    let dif = end.duration_since(beg).unwrap();
    println!("dynobj : {ret} : {dif:?}");

    let beg = SystemTime::now();
    let ret = dynobj::run(&dynobjs);
    let end = SystemTime::now();
    let dif = end.duration_since(beg).unwrap();
    println!("dynobj : {ret} : {dif:?}");

    let beg = SystemTime::now();
    let ret = enmobj::run(&enmobjs);
    let end = SystemTime::now();
    let dif = end.duration_since(beg).unwrap();
    println!("enmobj : {ret} : {dif:?}");

    let beg = SystemTime::now();
    let ret = enmobj::run(&enmobjs);
    let end = SystemTime::now();
    let dif = end.duration_since(beg).unwrap();
    println!("enmobj : {ret} : {dif:?}");
}
```

実行結果。

```console
$ cargo run 
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rust-polymorphism-bench`
dynobj : 2998794 : 11.085285ms
dynobj : 2998794 : 11.043607ms
enmobj : 2998794 : 12.997642ms
enmobj : 2998794 : 12.855571ms
```

なぜか debug ビルドでは Enum の方が遅い。。

```console
$ cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/rust-polymorphism-bench`
dynobj : 2999652 : 7.283027ms
dynobj : 2999652 : 7.243154ms
enmobj : 2999652 : 216.26µs
enmobj : 2999652 : 203.31µs
```

で release ビルドにすると Enum の方がメチャクチャ速くなる。

うーむ、、やっぱり関数ポインタの実行は最適化が難しくて遅くなるってことなのかな。。
こんなに違うとなると多少汚く(抽象化したいメソッド毎に `match` で分岐する関数を書くのがなんかアレ)なっても Enum の方がいいのかなあ。。
