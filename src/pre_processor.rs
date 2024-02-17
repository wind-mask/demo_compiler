use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex("//[^\n]*", logos::skip, priority = 100)]
    #[regex(r"\s+", logos::skip, priority = 100)]
    CommentOrWhitespace,
    // Start of a block comment.
    #[token("/*")]
    StartBlockComment,

    // End of a block comment.
    #[token("*/")]
    EndBlockComment,
    #[regex(r"\S+")]
    Other,
}

pub fn preprocess(code: &str) -> String {
    let mut output = String::new();
    let mut lexer = Token::lexer(code);
    let mut in_block_comment = false;
    while let Some(Ok(token)) = lexer.next() {
        match token {
            Token::StartBlockComment => {
                in_block_comment = true;
            }
            Token::EndBlockComment => {
                in_block_comment = false;
            }
            Token::Other => {
                if !in_block_comment {
                    output.push_str(lexer.slice());
                }
            }
            _ => {}
        }
    }
    if in_block_comment {
        panic!("Unterminated block comment");
    }

    output
}

#[cfg(test)]
#[test]
fn test_preprocessor() {
    use std::fs;
    use std::io::Write;
    let path = "R:\\CODE\\Compile_demo\\demo_compiler\\src\\test\\src.dsrc";
    let s = fs::read_to_string(path);
    let s = s.unwrap();
    let preprocessed = preprocess(s.as_str());
    let path = "R:\\CODE\\Compile_demo\\demo_compiler\\src\\test\\dst.dsrc";
    let mut file = fs::File::create(path).unwrap();
    file.write_all(preprocessed.as_bytes()).unwrap();
}
