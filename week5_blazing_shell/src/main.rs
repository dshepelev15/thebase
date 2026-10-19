use std::io::{Write, Error, ErrorKind};
use std::process::{Command, Stdio};


#[derive(PartialEq, Debug)]
struct CommandConfig {
    entrypoint: String,
    args: String,
}


fn execute_commands(configs: Vec<CommandConfig>) -> Result<String, Error> {
    let mut prev_stdout: Option<std::process::ChildStdout> = None;

    for (i, config) in configs.iter().enumerate() {
        let mut cmd = Command::new(&config.entrypoint);
        cmd.args(config.args.split_whitespace()).stdout(Stdio::piped());
        
        if let Some(prev) = prev_stdout.take() {
            cmd.stdin(prev);
        }

        let mut child = cmd.spawn()?;
        if i != configs.len() - 1 {
            prev_stdout = child.stdout.take();
            continue;
        }

        let output = child.wait_with_output()?;
        if output.status.success() {
            let result = String::from_utf8(output.stdout).unwrap();
            return Ok(result);
        } else {
            return Err(
                Error::new(
                    ErrorKind::Other,
                    format!("Process exited with status code {}", output.status.code().unwrap())
                ));
        }     
    }

    Ok(String::new())
}


fn parse_commands(command: &str) ->  Vec<CommandConfig> {
    let command = command.trim();
    if command.is_empty() {
        return Vec::new();
    }

    let parts = command.split("|");
    let mut result = Vec::new();
    for part in parts {
        let part = part.trim();
        if let Some(space_index) =  part.find(" ") {
            result.push(
                CommandConfig{
                    entrypoint: String::from(part[..space_index].trim()), 
                    args: String::from(part[space_index + 1..].trim())
                }
            )
        } else {
            result.push(
                CommandConfig{
                    entrypoint: String::from(part), 
                    args: String::from("")
                }
            )
        }        
    }
    result
}

const PROMPT: &str = "blazing_shell >>> ";

fn main() {
    loop {
        print!("{PROMPT}");

        std::io::stdout().flush().unwrap();
        let mut command_input = String::new();
        std::io::stdin().read_line(&mut command_input).unwrap();
        if command_input.trim() == "exit" {
            println!("Bye...");
            return;
        }
        let commands = parse_commands(&command_input);
        if commands.len() == 0 {
            continue;
        }
        let result = execute_commands(commands);
        match result {
            Ok(value) => println!("{value}"),
            Err(err) => {
                println!("Ошибка запуска '{}': {}", command_input.trim(), err.to_string());
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn is_same_vectors<T: PartialEq>(v1: Vec<T>, v2: Vec<T>) -> bool {
        if v1.len() != v2.len() {
            return false;
        }

        for (i, v) in v1.iter().enumerate() {
            if v != v2.get(i).unwrap() {
                return false
            }
        }

        true
    }

    #[test]
    fn test_parse_empty_command() {
        assert!(
            is_same_vectors(
                parse_commands("   "),
                vec![]
            )
        )
    }

    #[test]
    fn test_parse_one_command() {
        let command_input = " ls ";
        assert!(
            is_same_vectors(
                parse_commands(command_input),
                vec![
                    CommandConfig{
                    entrypoint: String::from("ls"),
                    args: String::from("")
                }]
            )
        );
    }

    #[test]
    fn test_parse_one_command_with_arguments() {
        let command_input = " ls -l -a ";
        assert!(
            is_same_vectors(
                parse_commands(command_input),
                vec![
                    CommandConfig{
                    entrypoint: String::from("ls"),
                    args: String::from("-l -a")
                }]
            )
        );
    }

    #[test]
    fn test_parse_multiple_commands() {
        let command_input = " ls -la | grep 'rust' | wc -l";
        assert!(
            is_same_vectors(
                parse_commands(command_input),
                vec![
                    CommandConfig{
                    entrypoint: String::from("ls"),
                    args: String::from("-la")
                },
                CommandConfig{
                    entrypoint: String::from("grep"),
                    args: String::from("'rust'")
                },
                CommandConfig{
                    entrypoint: String::from("wc"),
                    args: String::from("-l")
                }
                ]
            )
        );
    }

    #[test]
    fn test_execute_no_commands() {
        let result = execute_commands(vec![]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_execute_uknown_command() {
        let result = execute_commands(vec![
            CommandConfig{
                entrypoint: String::from("uknown_command"),
                args: String::from("--help")
            }
        ]);

        match result {
            Ok(_) => unreachable!(),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_execute_one_command() {
        let configs = vec![
            CommandConfig{
                entrypoint: String::from("tree"),
                args: String::from("--version")
            }
        ];
        let result = execute_commands(configs).unwrap();
        assert!(result.starts_with("tree v"));
    }

    #[test]
    fn test_execute_multiple_commands() {
        let configs = vec![
            CommandConfig{
                entrypoint: String::from("find"),
                args: String::from(". -type f -name *.rs")
            },
            CommandConfig{
                entrypoint: String::from("xargs"),
                args: String::from("cat")
            },
            CommandConfig{
                entrypoint: String::from("wc"),
                args: String::from("-l")
            }
        ];

        let result = execute_commands(configs).unwrap();
        let line_count = result.trim().parse::<i32>().unwrap();
        assert!(line_count > 0);
    }
}
