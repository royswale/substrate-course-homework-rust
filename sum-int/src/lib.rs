
pub fn sum(c: &[u32]) -> Option<u32> {
    let mut s : u32 = 0;
    let mut overflowing: bool = false;
    for u in c {
        match s.overflowing_add(*u) {
            (ss, false) => {
                s = ss;
            },
            (_, true) => {
                overflowing = true;
                break;
            }
        }
    }
    if overflowing {
        None
    } else {
        Some(s)
    }
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
