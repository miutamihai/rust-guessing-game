use std::cmp::Ordering;

pub enum MatchCase {
    Retry(Ordering),
    End
}

pub fn match_guess(guess: u32, secret_number: &u32) -> MatchCase {
    match guess.cmp(&secret_number) {
        Ordering::Less => MatchCase::Retry(Ordering::Less),
        Ordering::Greater => MatchCase::Retry(Ordering::Greater),
        Ordering::Equal => MatchCase::End
    }
}
