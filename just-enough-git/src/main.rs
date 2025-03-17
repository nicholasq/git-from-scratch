use std::env;
use std::fs;
use std::io::Write;
use std::str::FromStr;

const COMMAND_NAME: &str = "rgit";
const REPO_DIR: &str = ".rgit";

enum SubCommand {
    Init,
}

impl FromStr for SubCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "init" => Ok(SubCommand::Init),
            _ => Err(format!(
                "{}: '{}' is not a {} command.",
                COMMAND_NAME, s, COMMAND_NAME
            )),
        }
    }
}

impl std::fmt::Display for SubCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubCommand::Init => write!(f, "init"),
        }
    }
}

fn init(repo: &str) {
    let repo_dir = if repo == "." {
        env::current_dir().unwrap().join(REPO_DIR)
    } else {
        env::current_dir().unwrap().join(repo).join(REPO_DIR)
    };

    if !repo_dir.exists() {
        fs::create_dir_all(&repo_dir).unwrap();
        for dir in ["objects", "refs", "refs/heads"] {
            fs::create_dir(repo_dir.join(dir)).unwrap();
        }
        let mut head_file = fs::File::create_new(repo_dir.join("HEAD")).unwrap();
        write!(head_file, "ref: refs/heads/master").unwrap();
        println!(
            "initialized empty repository: {}",
            repo_dir.to_str().unwrap()
        );
    } else {
        println!(
            "Reinitialized existing Git repository in {}",
            repo_dir.to_str().unwrap()
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: {} <command>", COMMAND_NAME);
    } else {
        match &args[1].parse::<SubCommand>() {
            Ok(SubCommand::Init) => match args.len() {
                2 => init("."),
                3 => init(&args[2]),
                _ => println!("{}: 'init' takes at most 1 argument", COMMAND_NAME),
            },
            Err(msg) => println!("{}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cleanup() {
        let current_dir = env::current_dir().unwrap();
        let rgit_dir = current_dir.join(REPO_DIR);
        let test_repo_rgit_dir = current_dir.join("test_repo").join(REPO_DIR);

        if rgit_dir.exists() {
            fs::remove_dir_all(rgit_dir).unwrap();
        }

        if test_repo_rgit_dir.exists() {
            fs::remove_dir_all(test_repo_rgit_dir.parent().unwrap()).unwrap();
        }
    }

    #[test]
    fn test_init() {
        struct TestCase {
            name: &'static str,
            repo_path: &'static str,
        }

        let test_cases = [
            TestCase {
                name: "init current directory",
                repo_path: ".",
            },
            TestCase {
                name: "init specific directory",
                repo_path: "test_repo",
            },
        ];

        for test_case in &test_cases {
            cleanup();

            init(test_case.repo_path);

            let repo_dir = if test_case.repo_path == "." {
                env::current_dir().unwrap().join(REPO_DIR)
            } else {
                env::current_dir()
                    .unwrap()
                    .join(test_case.repo_path)
                    .join(REPO_DIR)
            };

            assert!(
                repo_dir.exists(),
                "{}: repo directory should exist",
                test_case.name
            );

            for dir in ["objects", "refs", "refs/heads"] {
                let dir_path = repo_dir.join(dir);
                assert!(
                    dir_path.exists(),
                    "{}: {} directory should exist",
                    test_case.name,
                    dir
                );
                assert!(
                    dir_path.is_dir(),
                    "{}: {} should be a directory",
                    test_case.name,
                    dir
                );
            }

            let head_path = repo_dir.join("HEAD");
            assert!(
                head_path.exists(),
                "{}: HEAD file should exist",
                test_case.name
            );
            assert!(
                head_path.is_file(),
                "{}: HEAD should be a file",
                test_case.name
            );

            let head_content = fs::read_to_string(head_path).unwrap();
            assert_eq!(
                head_content, "ref: refs/heads/master",
                "{}: HEAD should contain 'ref: refs/heads/master'",
                test_case.name
            );
        }

        cleanup();
    }
}
