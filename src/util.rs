pub fn is_chinese(words: &str) -> bool {
    for b in words.chars() {
        if b >= '\u{4E00}' && b <= '\u{9FA5}' {
            return true;
        }
    }
    return false;
}

#[test]
fn test_is_chinese() {
    assert_eq!(is_chinese("dsadsa"), false);
    assert_eq!(is_chinese("我是中文拉拉啊，he"), true);
}
