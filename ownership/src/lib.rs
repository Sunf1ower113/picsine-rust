
pub fn first_subword(mut s: String) -> String {
   for (i, c) in s.chars().enumerate() {
       if i != 0 && (c >= 'A' && c <= 'Z' || c == '_') {
           return s[..i].to_string()
       }
   }
    return s;
}