#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Round {
    player: Shape,
    opponent: Shape,
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Draw,
    Loss,
}

pub fn run(input: &str) {
    let lines = input.lines();

    let mut rounds = Vec::new();

    for line in lines {
        let mut shapes = line.split_whitespace();

        let opponent = match shapes.next().unwrap() {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid shape"),
        };

        let player = match shapes.next().unwrap() {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Invalid shape"),
        };

        rounds.push(Round { player, opponent });
    }

    let scores = rounds.iter().map(|round| {
        let round_result = get_round_result(round);

        let outcome_score = match round_result {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        };

        let shape_score = match round.player {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        return outcome_score + shape_score;
    });

    let total_score = scores.sum::<u32>();

    println!("{}", total_score);
}

fn get_round_result(round: &Round) -> RoundResult {
    let player = &round.player;
    let opponent = &round.opponent;

    if opponent == player {
        return RoundResult::Draw;
    }
    if (opponent == &Shape::Rock && player == &Shape::Paper)
        || (opponent == &Shape::Paper && player == &Shape::Scissors)
        || (opponent == &Shape::Scissors && player == &Shape::Rock)
    {
        return RoundResult::Win;
    }
    return RoundResult::Loss;
}
