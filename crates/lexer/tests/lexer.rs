#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use phprs_lexer::Token;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TokenCase {
    kind: String,
    #[serde(rename = "textLength")]
    text_length: usize,
}

fn variant_name(e: &Token) -> String {
    let name = format!("{:?}", e);
    if let Some(index) = name.find('(') {
        name[..index].to_string()
    } else {
        name.to_string()
    }
}

#[datatest::files("tests/cases", {
    input in r"^(.*).php$",
    output = r"${1}.php.tokens"
})]
fn run(input: &str, output: &str) {
    let tokens = phprs_lexer::lexer(input)
        .map(|(token, range)| match token {
            Ok(token) => (token, range),
            Err(_) => (Token::Unknown, range),
        })
        .collect::<Vec<_>>();

    let expected: Vec<TokenCase> = serde_json::from_str(output).unwrap();

    for (pos, e) in expected.iter().enumerate() {
        // if e.kind == "EndOfFileToken" {
        //     continue;
        // }

        let token = tokens.get(pos);

        let Some(token) = token else {
            panic!("Expected {} but found nothing.\n{:?}", e.kind, tokens)
        };

        let name = variant_name(&token.0);

        assert_eq!(name, e.kind.replace("Token", ""));
    }
}
