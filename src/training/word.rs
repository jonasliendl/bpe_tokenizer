#[derive(Clone)]
pub struct Word {
    pub letters: Vec<String>,
    pub occurence_count: usize,
}

impl Word {
    pub fn new(word: &str, occurence_count: Option<usize>) -> Self {
        // TODO: eventuall use word end indicator
        Word {
            occurence_count: match occurence_count {
                Some(cnt) => cnt,
                None => 1,
            },
            letters: word.chars().map(|char| char.to_string()).collect(),
        }
    }

    pub fn increase_occurence(&mut self) {
        self.occurence_count = self.occurence_count + 1;
    }

    pub fn merge_letters(&mut self, pair: (String, String)) {
        let mut merged = Vec::new();
        let mut i = 0;

        while i < self.letters.len() {
            if i < self.letters.len() - 1 && self.letters[i] == pair.0 && self.letters[i + 1] == pair.1 {
                // Merge the pair
                merged.push(format!("{}{}", pair.0, pair.1));
                i += 2; // skip the next token, since it was merged
            } else {
                merged.push(self.letters[i].clone());
                i += 1;
            }
        }

        self.letters = merged;
    }
}