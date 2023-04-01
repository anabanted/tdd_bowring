fn main() {
    let mut game = BowlingGame::new();
    for _i in 0..20 {
        game.new_shot(1);
    }
    println!("{}", game.calc_score())
}

#[derive(Debug)]
struct BowlingGame {
    frames: [Frame; 10],
}

#[derive(Debug, Clone, Copy)]
struct Frame {
    first_record: Shot,
    second_record: Shot,
}

#[derive(PartialEq, Eq)]
enum SpareOrStrike {
    Normal,
    Spare,
    Strike,
}

impl BowlingGame {
    fn new() -> BowlingGame {
        BowlingGame {
            frames: [Frame::new(); 10],
        }
    }
    fn calc_score(&self) -> i32 {
        let mut score = 0;
        for i in 0..self.frames.iter().len() {
            score += self.frames[i].calc_score();
            if self.frames[i].judge_spare_and_strike() == SpareOrStrike::Spare {
                score += self.spare_score(i);
            } else if self.frames[i].judge_spare_and_strike() == SpareOrStrike::Strike {
                score += self.strike_score(i);
            }
        }
        score
    }
    fn new_shot(&mut self, score: i32) {
        for i in 0..self.frames.iter().len() {
            if self.frames[i].has_notyet() {
                if self.frames[i].first_record == Shot::NotYet {
                    if score == 10 {
                        // Strike
                        self.frames[i].first_record = Shot::Pins(score);
                        self.frames[i].second_record = Shot::X;
                        break;
                    }
                    self.frames[i].first_record = Shot::Pins(score);
                } else {
                    self.frames[i].second_record = Shot::Pins(score);
                }
                break;
            }
        }
    }
    fn spare_score(&self, frame_num: usize) -> i32 {
        self.frames[frame_num + 1].first_record.to_score()
    }
    fn strike_score(&self, frame_num: usize) -> i32 {
        if self.frames[frame_num + 1].second_record == Shot::X {
            // sequential strike
            return self.frames[frame_num + 1].first_record.to_score()
                + self.frames[frame_num + 2].first_record.to_score();
        } else {
            self.frames[frame_num + 1].first_record.to_score()
                + self.frames[frame_num + 1].second_record.to_score()
        }
    }
}

impl Frame {
    fn new() -> Frame {
        Frame {
            first_record: Shot::NotYet,
            second_record: Shot::NotYet,
        }
    }
    fn calc_score(&self) -> i32 {
        self.first_record.to_score() + self.second_record.to_score()
    }
    fn has_notyet(&self) -> bool {
        if self.first_record == Shot::NotYet || self.second_record == Shot::NotYet {
            true
        } else {
            false
        }
    }
    fn judge_spare_and_strike(&self) -> SpareOrStrike {
        if self.first_record.to_score() == 10 {
            SpareOrStrike::Strike
        } else if self.calc_score() == 10 {
            SpareOrStrike::Spare
        } else {
            SpareOrStrike::Normal
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Shot {
    Pins(i32),
    X,
    NotYet,
}

impl Shot {
    fn to_score(&self) -> i32 {
        let score = match self {
            Shot::Pins(s) => *s,
            Shot::X => 0,
            Shot::NotYet => 0,
        };
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_gutter() {
        let mut game = BowlingGame::new();
        game.sequential_same_record(0, 20);
        assert_eq!(game.calc_score(), 0);
    }
    #[test]
    fn all_one_pin() {
        let mut game = BowlingGame::new();
        game.sequential_same_record(1, 20);
        assert_eq!(game.calc_score(), 20)
    }
    #[test]
    fn spare() {
        let mut game = BowlingGame::new();
        game.new_shot(3);
        game.new_shot(7);
        game.new_shot(4);
        assert_eq!(game.calc_score(), 18)
    }
    #[test]
    fn spare_shift() {
        let mut game = BowlingGame::new();
        game.new_shot(2);
        game.new_shot(5);
        game.new_shot(5);
        game.new_shot(1);
        game.sequential_same_record(0, 16);
        assert_eq!(game.calc_score(), 13);
    }
    #[test]
    fn strike() {
        let mut game = BowlingGame::new();
        game.new_shot(10);
        game.new_shot(3);
        game.new_shot(3);
        game.new_shot(1);
        game.sequential_same_record(0, 15);
        assert_eq!(game.calc_score(), 23)
    }
    #[test]
    fn double() {
        let mut game = BowlingGame::new();
        game.new_shot(10);
        game.new_shot(10);
        game.new_shot(3);
        game.new_shot(1);
        game.sequential_same_record(0, 14);
        assert_eq!(game.calc_score(), 41)
    }
    #[test]
    fn turkey() {
        let mut game = BowlingGame::new();
        game.new_shot(10);
        game.new_shot(10);
        game.new_shot(10);
        game.new_shot(3);
        game.new_shot(1);
        game.sequential_same_record(0, 12);
        assert_eq!(game.calc_score(), 71);
    }
    #[test]
    fn strike_and_spare() {
        let mut game = BowlingGame::new();
        game.new_shot(10);
        game.new_shot(5);
        game.new_shot(5);
        game.new_shot(3);
        game.sequential_same_record(0, 15);
        assert_eq!(game.calc_score(), 36);
    }
    #[test]
    fn double_and_spare() {
        let mut game = BowlingGame::new();
        game.new_shot(10);
        game.new_shot(10);
        game.new_shot(5);
        game.new_shot(5);
        game.new_shot(3);
        game.sequential_same_record(0, 13);
        assert_eq!(game.calc_score(), 61);
    }
    impl BowlingGame {
        fn sequential_same_record(&mut self, score: i32, times: i32) {
            for _i in 0..times {
                self.new_shot(score)
            }
        }
    }
}
