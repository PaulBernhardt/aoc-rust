use std::str::FromStr;

use common::{Problem, Solution};

struct Round {
    blue: u32,
    red: u32,
    green: u32,
}
impl FromStr for Round {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round = Round { blue: 0, red: 0, green: 0 };
        s.split(", ").for_each(|line| {
            let (count, color) = line.split_once(' ').unwrap();
            let count = count.parse::<u32>().unwrap();
            match color {
                "blue" => round.blue = count,
                "red" => round.red = count,
                "green" => round.green = count,
                _ => panic!("Unexpected colour"),
            }
        });
        Ok(round)
    }
}

struct Game {
    id: u32,
    power: u32,
    max: Round,
}

impl FromStr for Game {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rounds) = s.split_once(": ").unwrap();
        let id = id.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();
        let rounds: Vec<Round> =
            rounds.split("; ").map(|round| round.parse::<Round>().unwrap()).collect();

        let max = rounds.iter().fold(Round { blue: 0, green: 0, red: 0 }, |mut acc, e| {
            acc.blue = acc.blue.max(e.blue);
            acc.green = acc.green.max(e.green);
            acc.red = acc.red.max(e.red);
            acc
        });
        Ok(Game { id, power: max.blue * max.green * max.red, max })
    }
}

/// \--- Day 2: Cube Conundrum ---
/// ----------
///
/// You're launched high into the atmosphere! The apex of your trajectory just
/// barely reaches the surface of a large island floating in the sky. You gently
/// land in a fluffy pile of leaves. It's quite cold, but you don't see much
/// snow. An Elf runs over to greet you.
///
/// The Elf explains that you've arrived at *Snow Island* and apologizes for the
/// lack of snow. He'll be happy to explain the situation, but it's a bit of a
/// walk, so you have some time. They don't get many visitors up here; would you
/// like to play a game in the meantime?
///
/// As you walk, the Elf shows you a small bag and some cubes which are either
/// red, green, or blue. Each time you play this game, he will hide a secret
/// number of cubes of each color in the bag, and your goal is to figure out
/// information about the number of cubes.
///
/// To get information, once a bag has been loaded with cubes, the Elf will
/// reach into the bag, grab a handful of random cubes, show them to you, and
/// then put them back in the bag. He'll do this a few times per game.
///
/// You play several games and record the information from each game (your
/// puzzle input). Each game is listed with its ID number (like the `11` in
/// `Game 11: ...`) followed by a semicolon-separated list of subsets of cubes
/// that were revealed from the bag (like `3 red, 5 green, 4 blue`).
///
/// For example, the record of a few games might look like this:
///
/// ```
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
/// ```
///
/// In game 1, three sets of cubes are revealed from the bag (and then put back
/// again). The first set is 3 blue cubes and 4 red cubes; the second set is 1
/// red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green
/// cubes.
///
/// The Elf would first like to know which games would have been possible if the
/// bag contained *only 12 red cubes, 13 green cubes, and 14 blue cubes*?
///
/// In the example above, games 1, 2, and 5 would have been *possible* if the
/// bag had been loaded with that configuration. However, game 3 would have been
/// *impossible* because at one point the Elf showed you 20 red cubes at once;
/// similarly, game 4 would also have been *impossible* because the Elf showed
/// you 15 blue cubes at once. If you add up the IDs of the games that would
/// have been possible, you get `*8*`.
///
/// Determine which games would have been possible if the bag had been loaded
/// with only 12 red cubes, 13 green cubes, and 14 blue cubes. *What is the sum
/// of the IDs of those games?*
pub struct Day02;
impl Problem for Day02 {
    fn problem_input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn day(&self) -> u8 {
        2u8
    }
    fn name(&self) -> &str {
        "Day 2: Cube Conundrum"
    }
    fn solve_part1_with(&self, input: &str) -> Solution {
        Solution::U32(
            input
                .lines()
                .map(|s| s.parse::<Game>().unwrap())
                .filter(|game| game.max.red <= 12 && game.max.green <= 13 && game.max.blue <= 14)
                .fold(0u32, |total: u32, game: Game| total + game.id),
        )
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        Solution::U32(input.lines().map(|s| s.parse::<Game>().unwrap().power).sum())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let problem = Day02 {};
        assert_eq!(
            problem.solve_part1_with(
                "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
            ),
            Solution::U32(8)
        );
    }
    #[test]
    fn test_part1_real_input() {
        let problem = Day02 {};
        assert_eq!(problem.solve_part1(), Solution::U32(2331));
    }
    #[test]
    fn test_part2_example() {
        let problem = Day02 {};
        assert_eq!(
            problem.solve_part2_with(
                "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
            ),
            Solution::U32(2286)
        )
    }
    #[test]
    fn test_part2_real_input() {
        let problem = Day02 {};
        assert_eq!(problem.solve_part2(), Solution::U32(71585));
    }
}
