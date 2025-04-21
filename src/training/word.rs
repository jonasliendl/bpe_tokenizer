#[derive(Clone)]
pub struct Word {
    pub letters: Vec<usize>,
    pub occurence_count: usize,
}

impl Word {
    pub fn new(word: &str, occurence_count: Option<usize>) -> Self {
        let letters: Vec<usize> = word.as_bytes().to_vec().iter().map(|x| *x as usize).collect();
        Word {
            occurence_count: match occurence_count {
                Some(cnt) => cnt,
                None => 1,
            },
            letters,
        }
    }

    pub fn increase_occurence(&mut self) {
        self.occurence_count = self.occurence_count + 1;
    }

    pub fn merge_letters(&mut self, token_id: usize, pair: (usize, usize)) {
        let mut merged: Vec<usize> = Vec::new();
        let mut i = 0;

        while i < self.letters.len() {
            if i < self.letters.len() - 1 && self.letters[i] == pair.0 && self.letters[i + 1] == pair.1 {
                // Merge the pair
                merged.push(token_id);
                i += 2; // skip the next token, since it was merged
            } else {
                merged.push(self.letters[i].clone().into());
                i += 1;
            }
        }

        self.letters = merged;
    }
}
