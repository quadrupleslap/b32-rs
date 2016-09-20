pub fn encode(input: &str) -> String {
    let char_count = ((input.len() + 4) / 5) * 8;
    let mut result = String::with_capacity(char_count);
    let mut next_char: u8 = 0;
    let mut bits_left: u8 = 5;

    for b in input.as_bytes() {
        next_char |= b >> (8 - bits_left);
        result.push(
            if next_char < 26 {next_char + 65}
            else {next_char + 24} as char);
        
        if bits_left < 4 {
            next_char = b >> (3 - bits_left) & 31;
            result.push(
                if next_char < 26 {next_char + 65}
                else {next_char + 24} as char);
            bits_left += 5;
        }

        bits_left -= 3;
        next_char = (b << bits_left) & 31;
    }

    if result.len() < char_count {
        result.push(
            if next_char < 26 {next_char + 65}
            else {next_char + 24} as char);
        while result.len() < char_count {
            result.push('=');
        }
    }

    result
}

pub fn decode(input: &str) -> Option<String> {
    let actual_len = input.len()
        - input.as_bytes().iter().rev()
            .take_while(|&&b| b == '=' as u8).count();
    
    let byte_count = actual_len * 5 / 8;
    let mut result = String::with_capacity(byte_count);
    let mut cur_byte: u8 = 0;
    let mut bits_left: u8 = 8;
    let mut mask: u32;
    
    for &c in input.as_bytes().iter().take(actual_len) {
        let v =
            if c < 91 && c > 64 { c - 65 }
            else if c < 56 && c > 49 { c - 24 }
            else if c < 123 && c > 96 { c - 97 }
            else {return None} as u32;
            
        if bits_left > 5 {
            mask = v << (bits_left - 5);
            cur_byte |= mask as u8;
            bits_left -= 5;
        } else {
            mask = v >> (5 - bits_left);
            cur_byte |= mask as u8;
            result.push(cur_byte as char);
            bits_left += 3;
            cur_byte = (v << bits_left) as u8;
        }
    }

    if result.len() < byte_count {
        result.push(cur_byte as char);
    }        

    Some(result)
}

