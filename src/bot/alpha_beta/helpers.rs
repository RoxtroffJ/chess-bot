/// Compares the scores and returns the duo with minimal if ```min``` and maximal otherwise score.
/// 
/// # Exemple
/// ```ignore
/// # use chess_bot::bot::alpha_beta::*;
/// # use chess_bot::bot::*;
///
/// let mut score = 2;
/// let mut val = "foo";
/// 
/// update_with_scores(&mut score, &mut val, 3, "bar", false);
/// assert_eq!(score, 3);
/// assert_eq!(val, "bar");
/// 
/// update_with_scores(&mut score, &mut val, 2, "foo", false);
/// assert_eq!(score, 3);
/// assert_eq!(val, "bar");
/// 
/// update_with_scores(&mut score, &mut val, 3, "bar", true);
/// assert_eq!(score, 3);
/// assert_eq!(val, "bar");
/// 
/// update_with_scores(&mut score, &mut val, 2, "foo", true);
/// assert_eq!(score, 2);
/// assert_eq!(val, "foo");
/// 
/// ```
pub(super) fn update_with_scores<S: Ord + Copy, T>(score: &mut S, val: &mut T, other_score: S, other_val: T, min: bool) {
    if if min {*score > other_score} else {*score < other_score} {
        *score = other_score;
        *val = other_val;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_with_scores() {
        let mut score = 2;
        let mut val = "foo";

        update_with_scores(&mut score, &mut val, 3, "bar", false);
        assert_eq!(score, 3);
        assert_eq!(val, "bar");

        update_with_scores(&mut score, &mut val, 2, "foo", false);
        assert_eq!(score, 3);
        assert_eq!(val, "bar");

        update_with_scores(&mut score, &mut val, 3, "bar", true);
        assert_eq!(score, 3);
        assert_eq!(val, "bar");

        update_with_scores(&mut score, &mut val, 2, "foo", true);
        assert_eq!(score, 2);
        assert_eq!(val, "foo");
    }
}