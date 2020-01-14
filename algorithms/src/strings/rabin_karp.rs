pub fn rabin_karp(pattern:String, text:String) -> bool{
    if pattern.len() == 0 {
        return false
    }
    let size = text.len();
    if size == 0 {
        return false
    }
    let pattern_hash = hash(pattern.as_str());
    let pattern_length = pattern.len();
    for n in 0..(size - pattern_length - 1) {
        let slice = &text[n..n + pattern_length];
        let hash_result = hash(slice);
        if hash_result == pattern_hash && slice == pattern {
            return true
        }
    }
    return false

}

fn hash(s:&str) -> i32 {
    return 0
}