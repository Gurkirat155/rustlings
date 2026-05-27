fn elems_to_vec(a: i32, b: i32, c: i32) -> Vec<i32> {
    // TODO: Return a vector containing the elements a, b and c (in this order).
    // Use the "vec!" macro.

    // Solution 1
    // let mut val: Vec<i32> = Vec::new();
    // val.push(a);
    // val.push(b);
    // val.push(c);
    // val

    vec![a,b,c]
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elems_to_vec() {
        let (a, b, c) = (2, 7, 12);
        let v = elems_to_vec(a, b, c);
        assert_eq!(v[0], a);
        assert_eq!(v[1], b);
        assert_eq!(v[2], c);
    }
}
