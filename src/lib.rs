/// Standard Rectified Linear Unit (ReLU) function.
///
/// See also: https://medium.com/tinymind/a-practical-guide-to-relu-b83ca804f1f7
///
/// # Examples
///
/// ```
/// assert!(relu(3.14) == 3.14)
/// assert!(relu(-1.9) == 0.0)
/// ```
fn relu(x: f32) -> f32 {
    x.max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relu_pos() {
        assert_eq!(relu(1.0), 1.0)
    }
    fn relu_neg() {
        assert_eq!(relu(-1.0), 0.0)
    }
}
