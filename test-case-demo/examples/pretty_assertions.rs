fn main() {

}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_example() {
        let expected = vec![1, 2, 3];
        let actual = vec![1, 2, 4]; // 注意这里故意让测试失败

        assert_eq!(expected, actual);
    }

}