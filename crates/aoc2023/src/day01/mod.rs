use common::{Problem, Solution};
/// \--- Day 1: Trebuchet?! ---
/// ----------
///
/// Something is wrong with global snow production, and you've been selected to
/// take a look. The Elves have even given you a map; on it, they've used stars
/// to mark the top fifty locations that are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations,
/// you need to check all *fifty stars* by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each
/// day in the Advent calendar; the second puzzle is unlocked when you complete
/// the first. Each puzzle grants *one star*. Good luck!
///
/// You try to ask why they can't just use a [weather machine](/2015/day/1) ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a [trebuchet](https://en.wikipedia.org/wiki/Trebuchet) ("please hold still, we need to strap you in").
///
/// As they're making the final adjustments, they discover that their
/// calibration document (your puzzle input) has been *amended* by a very young
/// Elf who was apparently just excited to show off her art skills.
/// Consequently, the Elves are having trouble reading the values on the
/// document.
///
/// The newly-improved calibration document consists of lines of text; each line
/// originally contained a specific *calibration value* that the Elves now need
/// to recover. On each line, the calibration value can be found by combining
/// the *first digit* and the *last digit* (in that order) to form a single
/// *two-digit number*.
///
/// For example:
///
/// ```
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
/// ```
///
/// In this example, the calibration values of these four lines are `12`, `38`,
/// `15`, and `77`. Adding these together produces `*142*`.
///
/// Consider your entire calibration document. *What is the sum of all of the
/// calibration values?*
pub struct Day01;
impl Problem for Day01 {
    fn problem_input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn day(&self) -> u8 {
        1u8
    }
    fn name(&self) -> &str {
        "Day 1: Trebuchet?!"
    }
    fn solve_part1_with(&self, input: &str) -> Solution {
        Solution::U32(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .find(|&c| c.is_numeric())
                        .map_or(0, |c| c.to_digit(10).expect("Bad left number"))
                        * 10
                        + line
                            .chars()
                            .rfind(|&c| c.is_numeric())
                            .map_or(0, |c| c.to_digit(10).expect("Bad right number"))
                })
                .sum(),
        )
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        let pairs = vec![
            ("one", "o1e"),
            ("two", "t2o"),
            ("three", "t3e"),
            ("four", "f4r"),
            ("five", "f5e"),
            ("six", "s6x"),
            ("seven", "s7n"),
            ("eight", "e8t"),
            ("nine", "n9e"),
            ("zero", "z0e"),
        ];
        Solution::U32(
            input
                .lines()
                .map(|line| {
                    pairs.iter().fold(line.to_string(), |l, (word, num)| l.replace(word, num))
                })
                .map(|line| {
                    line.chars()
                        .find(|&c| c.is_numeric())
                        .map_or(0, |c| c.to_digit(10).expect("Bad left number"))
                        * 10
                        + line
                            .chars()
                            .rfind(|&c| c.is_numeric())
                            .map_or(0, |c| c.to_digit(10).expect("Bad right number"))
                })
                .sum(),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let problem = Day01 {};
        assert_eq!(
            problem.solve_part1_with(
                "\
        1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            Solution::U32(142)
        );
    }
    #[test]
    fn test_part1_real_input() {
        let problem = Day01 {};
        assert_eq!(problem.solve_part1(), Solution::U32(55607));
    }
    #[test]
    fn test_part2_example() {
        let problem = Day01 {};
        assert_eq!(
            problem.solve_part2_with(
                "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            Solution::U32(281)
        );
    }
    #[test]
    fn test_part2_real_input() {
        let problem = Day01 {};
        assert_eq!(problem.solve_part2(), Solution::U32(55291));
    }
}
