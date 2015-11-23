extern crate chamkho;

#[cfg(test)]
mod tests {
    use chamkho::dict::Dict;
    use chamkho::wordcut::Wordcut;
    use chamkho::graph::TextRange;
    use std::path::Path;

    #[test]
    fn test_default_dict() {
        let dict = Dict::load(Dict::default_path());
        assert!(dict.is_ok());
        assert!(dict.unwrap_or_else(|e| panic!("Cannot open dict {}", e)).r() > 10000);
    }

    #[test]
    fn test_vec_dict() {
        let dict = Dict::create(&vec!["ก".to_string(), "กา".to_string()]);
        assert!(dict.is_ok());
        assert!(dict.unwrap().r() == 1);
    }

    #[test]
    fn test_segment_into_ranges() {
        let dict = Dict::load(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let ranges = wordcut.segment(&"กากกา".to_string());
        let expected = vec![TextRange{s:0,e:3}, TextRange{s:3,e:5}];
        assert_eq!(ranges, expected)
    }

    #[test]
    fn test_segment_into_strings() {
        let dict = Dict::load(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings(&"กากกา".to_string());
        let expected = vec!["กาก".to_string(), "กา".to_string()];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_segment_with_punc() {
        let dict = Dict::load(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment_into_strings(&"\"กากา\"".to_string());
        let expected = &vec!["\"", "กา", "กา", "\"",]
            .iter()
            .map(|&s| s.to_string()).collect::<Vec<String>>()[..];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_space() {
        let dict = Dict::load(Path::new("data/thai.txt"));
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment(&"a cat".to_string());
        let expected = vec![TextRange{s:0,e:1},TextRange{s:1,e:2},TextRange{s:2,e:5}];
        assert_eq!(words, expected)
    }

    #[test]
    fn test_lao() {
        let dict = Dict::load(Dict::lao_path());
        let wordcut = Wordcut::new(dict.unwrap());
        let words = wordcut.segment(&"ພາສາລາວມີ".to_string());
        let expected = vec![TextRange{s:0,e:4},TextRange{s:4,e:7},TextRange{s:7,e:9}];
        assert_eq!(words, expected)
    }
}
