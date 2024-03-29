/*! 这个是一个简单的统计文本中单词出现的次数的程序
 * 
 */

// 包和模块级别的注释，可以使用 /*! */ 或 //! ，注意一种方式感叹号前只有个星号

use regex::Regex;

/// 单词出现的次数
#[derive(Debug)]
pub struct WordCount {
    /// 单词
   pub word: String,
   /// 出现的次数
   pub count: usize,
}

impl PartialEq for WordCount {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word && self.count == other.count
    }
}

/**
 * 统计文本中单词出现的次数
 * 
 * * `text` - 要统计的文本
 */
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
                word: "s".to_string(),
                count: 1,
            },
            WordCount {
                word: "it".to_string(),
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
        assert!(is_equal(&result, &expected));
    }

    fn is_equal(a: &Vec<WordCount>, b: &Vec<WordCount>) -> bool {
        if a.len() != b.len() {
            return false;
        }
        for i in 0..a.len() {
            let item = b.iter().find(|&x| x == &a[i]);
            if item.is_none() {
                return false;
            }
        }
        true
    }
}
