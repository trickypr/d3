use logos::{Lexer, Logos};
use orbtk::prelude::*;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex("#.*\n")]
    Comment,

    #[token("\n")]
    Newline,

    #[token("exec")]
    Exec,

    #[token("exec_always")]
    ExecAlways,

    #[regex(r"--[a-zA-Z0-9_\-]*")]
    CmdArg,

    #[regex("\\$[a-zA-Z0-9_]+")]
    Variable,

    #[regex("\"[a-zA-Z0-9_ ]*\"")]
    String,

    #[regex(r"(/|~|(~/)|(\./))[-a-zA-Z0-9_.&/=]*")]
    Path,

    #[regex("[a-zA-Z0-9_]+")]
    Identifier,

    #[token("{")]
    CurlyOpen,

    #[token("}")]
    CurlyClosed,

    #[token("=")]
    Equals,

    #[token("[")]
    SquareOpen,

    #[token("]")]
    SquareClosed,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,
}

pub fn tokenize(input: &str) -> Lexer<Token> {
    Token::lexer(input)
}

pub type ConfigExecType = (Vec<String>, String);

#[derive(Debug, PartialEq, Clone, AsAny)]
pub enum ConfigTree {
    Comment(String),
    Newline,
    /// First item is the arguments (see `CmdArg`), second is the value is the
    /// path
    Exec(ConfigExecType),
    /// First item is the arguments (see `CmdArg`), second is the value is the
    /// path
    ExecAlways(ConfigExecType),
    Unknown(String),
}

into_property_source!(ConfigTree);

fn exec(input: &mut Lexer<Token>) -> ConfigExecType {
    let mut args = Vec::new();
    let mut path = String::new();
    while let Some(token) = input.next() {
        match token {
            Token::CmdArg => args.push(input.slice().to_string()),
            Token::Path => path = input.slice().to_string(),
            Token::Identifier => path = input.slice().to_string(),

            _ => break,
        }
    }
    (args, path)
}

pub type Config = Vec<ConfigTree>;

pub fn parse(mut input: Lexer<Token>) -> Config {
    let mut config_tree = Vec::new();

    let mut done = false;

    while !done {
        if let Some(token) = input.next() {
            match token {
                Token::Comment => {
                    let comment = input.slice();
                    config_tree.push(ConfigTree::Comment(comment[1..comment.len()].to_string()));
                }
                Token::Newline => config_tree.push(ConfigTree::Newline),
                Token::Exec => config_tree.push(ConfigTree::Exec(exec(&mut input))),
                Token::ExecAlways => config_tree.push(ConfigTree::ExecAlways(exec(&mut input))),
                Token::CmdArg => config_tree.push(ConfigTree::Unknown(input.slice().to_string())),
                Token::Variable => config_tree.push(ConfigTree::Unknown(input.slice().to_string())),
                Token::String => config_tree.push(ConfigTree::Unknown(input.slice().to_string())),
                Token::Path => config_tree.push(ConfigTree::Unknown(input.slice().to_string())),
                Token::Identifier => {
                    config_tree.push(ConfigTree::Unknown(input.slice().to_string()))
                }
                Token::CurlyOpen => {
                    config_tree.push(ConfigTree::Unknown(input.slice().to_string()))
                }
                Token::CurlyClosed => {
                    config_tree.push(ConfigTree::Unknown(input.slice().to_string()))
                }
                Token::Equals => config_tree.push(ConfigTree::Unknown(input.slice().to_string())),
                Token::SquareOpen => {
                    config_tree.push(ConfigTree::Unknown(input.slice().to_string()))
                }
                Token::SquareClosed => {
                    config_tree.push(ConfigTree::Unknown(input.slice().to_string()))
                }
                Token::Error => config_tree.push(ConfigTree::Unknown(input.slice().to_string())),
            };
        } else {
            done = true;
        }
    }

    config_tree
}

pub fn compile(input: &str) -> Config {
    parse(tokenize(input))
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn lexer() {
        let mut lex = tokenize("# TEST \n\nexec --no-startup-id /usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1\nexec_always --no-startup-id fix_xcursor");

        assert_eq!(lex.next(), Some(Token::Comment));
        assert_eq!(lex.slice(), "# TEST \n");

        assert_eq!(lex.next(), Some(Token::Newline));

        assert_eq!(lex.next(), Some(Token::Exec));

        assert_eq!(lex.next(), Some(Token::CmdArg));
        assert_eq!(lex.slice(), "--no-startup-id");

        assert_eq!(lex.next(), Some(Token::Path));
        assert_eq!(
            lex.slice(),
            "/usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1"
        );

        assert_eq!(lex.next(), Some(Token::Newline));

        assert_eq!(lex.next(), Some(Token::ExecAlways));

        assert_eq!(lex.next(), Some(Token::CmdArg));
        assert_eq!(lex.slice(), "--no-startup-id");

        assert_eq!(lex.next(), Some(Token::Identifier));
        assert_eq!(lex.slice(), "fix_xcursor");
    }

    #[test]
    fn parser() {
        let lex = tokenize("# TEST \n\nexec --no-startup-id /usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1\nexec_always --no-startup-id fix_xcursor");
        let parser_tree = parse(lex);

        assert_eq!(
            parser_tree,
            vec![
                ConfigTree::Comment(" TEST \n".to_string()),
                ConfigTree::Newline,
                ConfigTree::Exec((
                    vec!["--no-startup-id".to_string()],
                    "/usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1".to_string()
                )),
                ConfigTree::ExecAlways((
                    vec!["--no-startup-id".to_string()],
                    "fix_xcursor".to_string()
                )),
            ]
        )
    }
}
