use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct WordCount {
    word: String,
    count: usize,
}

pub fn statistics(text: &str) -> Vec<WordCount> {
    let mut words: Vec<WordCount> = Vec::new();
    let re = Regex::new(r"\w+").unwrap();

    for word in re.captures_iter(text) {
        let word = word.get(0).unwrap().as_str();

        let mut found = false;
        for w in &mut words {
            if w.word == word {
                w.count += 1;
                found = true;
                break;
            }
        }
        if !found {
            words.push(WordCount {
                word: word.to_string(),
                count: 1,
            });
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = statistics(
            "
        a a b b c
        a
    ",
        );

        let expected = vec![
            WordCount {
                word: "a".to_string(),
                count: 3,
            },
            WordCount {
                word: "b".to_string(),
                count: 2,
            },
            WordCount {
                word: "c".to_string(),
                count: 1,
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works2() {
        let result = statistics(
            "
         it's a test
    ",
        );

        let expected = vec![
            WordCount {
                word: "it".to_string(),
                count: 1,
            },
            WordCount {
                word: "s".to_string(),
                count: 1,
            },
            WordCount {
                word: "a".to_string(),
                count: 1,
            },
            WordCount {
                word: "test".to_string(),
                count: 1,
            },
        ];
        assert_eq!(result, expected);
    }
}

// todo compare array
