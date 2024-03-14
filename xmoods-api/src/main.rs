fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_test() {
        assert!(2+2 == 4)
    }
}
