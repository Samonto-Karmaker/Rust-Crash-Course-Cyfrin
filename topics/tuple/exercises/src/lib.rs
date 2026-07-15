pub fn first(t: (bool, u32, char)) -> bool {
    let (x, _, _) = t;
    x
}

pub fn last(t: (bool, u32, char)) -> char {
    t.2
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    let (x, y) = t;
    (y, x)
    // (t.1, t.0)
}
