pub fn rabin_karp(pattern:String, text:String) -> bool{
    if pattern.length() == 0 {
        false
    }
    if text.length() == 0 {
        false
    }
    pattern_hash = hash(pattern)
    pattern_length = pattern.length()
    for n in text.length() - pattern_length - 1 {
        slice = text[n:n + pattern_length]
        hash_result = hash(slice)
        if hash_result == pattern_hash && slice == pattern {
            true
        }
    }
    false

}

fn hash(s:String) -> int {
    return 0
}