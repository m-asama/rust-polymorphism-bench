//

pub struct EnmObj1 {}

impl EnmObj1 {
    fn ret(&self) -> usize {
        1
    }
}

pub struct EnmObj2 {}

impl EnmObj2 {
    fn ret(&self) -> usize {
        2
    }
}

pub struct EnmObj3 {}

impl EnmObj3 {
    fn ret(&self) -> usize {
        3
    }
}

pub struct EnmObj4 {}

impl EnmObj4 {
    fn ret(&self) -> usize {
        4
    }
}

pub struct EnmObj5 {}

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

impl EnmObj {
    fn ret(&self) -> usize {
        match &self {
            EnmObj::EnmObj1(x) => x.ret(),
            EnmObj::EnmObj2(x) => x.ret(),
            EnmObj::EnmObj3(x) => x.ret(),
            EnmObj::EnmObj4(x) => x.ret(),
            EnmObj::EnmObj5(x) => x.ret(),
        }
    }
}

pub fn build(rands: &[usize]) -> Vec<EnmObj> {
    let mut v: Vec<EnmObj> = vec![];
    for r in rands {
        match r {
            1 => v.push(EnmObj::EnmObj1(EnmObj1 {})),
            2 => v.push(EnmObj::EnmObj2(EnmObj2 {})),
            3 => v.push(EnmObj::EnmObj3(EnmObj3 {})),
            4 => v.push(EnmObj::EnmObj4(EnmObj4 {})),
            5 => v.push(EnmObj::EnmObj5(EnmObj5 {})),
            _ => panic!(""),
        }
    }
    v
}

pub fn run(enmobjs: &Vec<EnmObj>) -> usize {
    let mut x = 0;
    for enmobj in enmobjs {
        x += enmobj.ret();
    }
    x
}
