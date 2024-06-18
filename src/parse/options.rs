use crate::network::Error;

use super::Input;

pub type ParseOption<T> = fn(&mut T, &str, &mut Input) -> Result<(), Error>;
pub type Options<T> = Vec<(Vec<&'static str>, ParseOption<T>)>;

pub fn parse_options<T>(
    command: &str,
    options: &Vec<(Vec<&'static str>, ParseOption<T>)>,
    input: &mut Input,
    mut target: T,
) -> Result<T, Error> {
    let mut used_ops = Vec::new();

    'outer: while input.has_next() {
        let token = input.next_token().unwrap();

        println!("token: {}", token);
        for (ref tokens, ref op) in options {
            println!("tokens: {:?}", tokens);
            if tokens.contains(&token.as_str()) {
                if used_ops.contains(&op) {
                    return Err(Error::DuplicateOption(command.to_string(), token));
                }

                used_ops.push(op);
                op(&mut target, &token, input)?;
                continue 'outer;
            }
        }

        return Err(Error::UnknownOption(command.to_string(), token));
    }

    Ok(target)
}
