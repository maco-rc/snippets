struct Encrypt {
    message: String,
}

impl Encrypt {
    fn new(message: String) -> Self {
        Encrypt { message: message }
    }

    fn encrypt(m: String) -> String {
        let encrypt = Encrypt::new(m);

        let res = encrypt
            .message
            .split_whitespace()
            .map(|item| {
                let mut chars: Vec<_> = item.chars().collect();
                let ascii = chars.remove(0);

                match item.len() {
                    0 => unreachable!(),
                    1 => return format!("{}", ascii as u32),
                    2 => return format!("{}{}", ascii as u32, chars.into_iter().collect::<String>()),
                    _ => {
                        chars.swap(0, item.len() - 2);
                        return format!("{}{}", ascii as u32, chars.into_iter().collect::<String>())
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        
        res
    }
}

fn encrypt_this(text: &str) -> String {
    Encrypt::encrypt(text.to_string())
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(encrypt_this(&"A"), "65".to_string());
        assert_eq!(
            encrypt_this(&"A wise old owl lived in an oak"),
            "65 119esi 111dl 111lw 108dvei 105n 97n 111ka".to_string()
        );
        assert_eq!(
            encrypt_this(&"The more he saw the less he spoke"),
            "84eh 109ero 104e 115wa 116eh 108sse 104e 115eokp".to_string()
        );
        assert_eq!(
            encrypt_this(&"The less he spoke the more he heard"),
            "84eh 108sse 104e 115eokp 116eh 109ero 104e 104dare".to_string()
        );
        assert_eq!(
            encrypt_this(&"Why can we not all be like that wise old bird"),
            "87yh 99na 119e 110to 97ll 98e 108eki 116tah 119esi 111dl 98dri".to_string()
        );
        assert_eq!(
            encrypt_this(&"Thank you Piotr for all your help"),
            "84kanh 121uo 80roti 102ro 97ll 121ruo 104ple".to_string()
        );
    }
}
