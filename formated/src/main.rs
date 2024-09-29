use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena,
};
use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a filename as an argument.");
        std::process::exit(1);
    }

    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let arena = Arena::new();
    let root = parse_document(&arena, &contents, &comrak::ComrakOptions::default());
    let headers = get_headers(root);

    let result = if headers.len() == 4
        && headers[0] == "Question"
        && headers[1] == "Approach"
        && headers[2] == "Solution"
        && headers[3] == "Summary"
    {
        Ok("Format Accepted 👍")
    } else {
        Err("Format Rejected ❌")
    };

    match result {
        Ok(message) => println!("{}", message),
        Err(error) => println!("{}", error),
    }

    Ok(())
}

fn get_headers<'a>(node: &'a AstNode<'a>) -> Vec<String> {
    node.descendants()
        .filter_map(|n| {
            if let NodeValue::Heading(_) = n.data.borrow().value {
                Some(
                    n.first_child()
                        .and_then(|child| {
                            if let NodeValue::Text(ref text) = child.data.borrow().value {
                                Some(String::from_utf8_lossy(text).into_owned())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default(),
                )
            } else {
                None
            }
        })
        .collect()
}
