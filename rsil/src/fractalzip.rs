#[derive(Default, Debug)]
struct Node {
    children: Vec<usize>,
    data: Vec<u8>,
}

pub struct FractalZip;

impl FractalZip {
    pub fn new() -> Self {
        FractalZip
    }

    pub fn compress(&self, raw_text: &str) -> Vec<u8> {
        let words = tokenize(raw_text);

        let mut unique_words = Vec::new();
        let mut word_set = std::collections::HashSet::new();

        for word in &words {
            if !word_set.contains(word) {
                unique_words.push(word.clone());
                word_set.insert(word.clone());
            }
        }

        let mut transmission = vec![2]; // BRANCH IN (Start Dictionary Sphere)
        for word in &unique_words {
            transmission.push(2); // Branch IN (Start Word Definition)
            transmission.extend(text_to_binary(word));
            transmission.push(3); // Branch OUT (End Word Definition)
        }
        transmission.push(3); // BRANCH OUT (End Dictionary Sphere)

        transmission.push(2); // BRANCH IN (Start Payload Sphere)
        for word in &words {
            let word_id = unique_words.iter().position(|w| w == word).unwrap();
            
            transmission.push(2); // Branch IN (Start Pointer)

            if word_id == 0 {
                transmission.push(0);
            } else {
                let mut bin_id = Vec::new();
                let mut temp_id = word_id;
                while temp_id > 0 {
                    bin_id.insert(0, (temp_id % 2) as u8);
                    temp_id /= 2;
                }
                transmission.extend(bin_id);
            }

            transmission.push(3); // Branch OUT (End Pointer)
        }
        transmission.push(3); // BRANCH OUT (End Payload Sphere)

        transmission
    }

    pub fn decompress(&self, stream: &[u8]) -> String {
        let mut nodes: Vec<Node> = vec![Node::default()];
        let mut stack: Vec<usize> = vec![0];
        let mut current: usize = 0;

        for &symbol in stream {
            if symbol == 0 || symbol == 1 {
                nodes[current].data.push(symbol);
            } else if symbol == 2 {
                let new_node_idx = nodes.len();
                nodes.push(Node::default());
                nodes[current].children.push(new_node_idx);
                stack.push(new_node_idx);
                current = new_node_idx;
            } else if symbol == 3 {
                if stack.len() > 1 {
                    stack.pop();
                    current = *stack.last().unwrap();
                }
            }
        }

        if nodes[0].children.len() != 2 {
            return String::new(); // Invalid stream
        }

        let dict_sphere_idx = nodes[0].children[0];
        let payload_sphere_idx = nodes[0].children[1];

        let mut rebuilt_dictionary = std::collections::HashMap::new();
        for (i, &word_node_idx) in nodes[dict_sphere_idx].children.iter().enumerate() {
            let word = binary_to_text(&nodes[word_node_idx].data);
            rebuilt_dictionary.insert(i, word);
        }

        let mut final_text = String::new();
        for &pointer_node_idx in nodes[payload_sphere_idx].children.iter() {
            let mut pointer_id = 0;
            for &b in &nodes[pointer_node_idx].data {
                pointer_id = (pointer_id << 1) | (b as usize);
            }
            if let Some(word) = rebuilt_dictionary.get(&pointer_id) {
                final_text.push_str(word);
            }
        }

        final_text
    }
}

pub fn text_to_binary(text: &str) -> Vec<u8> {
    let mut bin = Vec::new();
    for byte in text.as_bytes() {
        for i in (0..=7).rev() {
            let bit = (byte >> i) & 1;
            bin.push(bit);
        }
    }
    bin
}

pub fn binary_to_text(bin_array: &[u8]) -> String {
    let mut bytes = Vec::new();
    for i in (0..bin_array.len()).step_by(8) {
        let mut b: u8 = 0;
        for j in 0..8 {
            if i + j < bin_array.len() && bin_array[i + j] == 1 {
                b |= 1 << (7 - j);
            }
        }
        bytes.push(b);
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    let mut state = 0; // 0=start, 1=word, 2=space, 3=other

    for c in text.chars() {
        let current_state = if c.is_alphanumeric() {
            1
        } else if c.is_whitespace() {
            2
        } else {
            3
        };

        if state != 0 && state != current_state {
            tokens.push(current.clone());
            current.clear();
        }
        current.push(c);
        state = current_state;
    }
    
    if !current.is_empty() {
        tokens.push(current);
    }
    
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_to_binary() {
        let bin = text_to_binary("A");
        // 'A' is 65 -> 01000001
        assert_eq!(bin, vec![0, 1, 0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn test_binary_to_text() {
        let bin = vec![0, 1, 0, 0, 0, 0, 0, 1];
        assert_eq!(binary_to_text(&bin), "A");
    }

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("Hello, world! 123");
        assert_eq!(tokens, vec!["Hello", ",", " ", "world", "!", " ", "123"]);
    }

    #[test]
    fn test_compress_decompress() {
        let fz = FractalZip::new();
        let target_text = "the alien sent the signal and the alien waited for the signal";
        let payload = fz.compress(target_text);
        let recovered_text = fz.decompress(&payload);
        assert_eq!(target_text, recovered_text);
    }

    #[test]
    fn test_lossless_compress_decompress() {
        let fz = FractalZip::new();
        let target_text = "Hello, World!\nThis is a lossless test.\n\n\tTabs,   spaces, and newlines.";
        let payload = fz.compress(target_text);
        let recovered_text = fz.decompress(&payload);
        assert_eq!(target_text, recovered_text);
    }
}
