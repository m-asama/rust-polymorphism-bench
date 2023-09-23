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
    /*
    for x in &rands {
        println!("{}", x);
    }
    */
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
