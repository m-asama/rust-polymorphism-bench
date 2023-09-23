//

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

impl DynObj for DynObj2 {
    fn ret(&self) -> usize {
        2
    }
}

struct DynObj3 {}

impl DynObj for DynObj3 {
    fn ret(&self) -> usize {
        3
    }
}

struct DynObj4 {}

impl DynObj for DynObj4 {
    fn ret(&self) -> usize {
        4
    }
}

struct DynObj5 {}

impl DynObj for DynObj5 {
    fn ret(&self) -> usize {
        5
    }
}

pub fn build(rands: &[usize]) -> Vec<Box<dyn DynObj>> {
    let mut v: Vec<Box<dyn DynObj>> = vec![];
    for r in rands {
        match r {
            1 => v.push(Box::new(DynObj1 {})),
            2 => v.push(Box::new(DynObj2 {})),
            3 => v.push(Box::new(DynObj3 {})),
            4 => v.push(Box::new(DynObj4 {})),
            5 => v.push(Box::new(DynObj5 {})),
            _ => panic!(""),
        }
    }
    v
}

pub fn run(dynobjs: &Vec<Box<dyn DynObj>>) -> usize {
    let mut x = 0;
    for dynobj in dynobjs {
        x += dynobj.ret();
    }
    x
}
