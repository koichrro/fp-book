#[derive(Debug, PartialEq)]
struct Point(i32, i32);

#[cfg(test)]
mod tests {
    use crate::cb_5_15::Point;

    #[test]
    fn test_flatmap() {
        let xs = vec![1];
        let ys = vec![-2, 7];

        let ret: Vec<Point> = xs.iter().flat_map(|x| {
            ys.iter().map(|y| {
                Point(x.to_owned(), y.to_owned())
            })
        }).collect();

        assert_eq!(ret, vec![Point(1, -2), Point(1, 7)]);
    }
}