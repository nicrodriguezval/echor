use clap::{App, Arg};

fn main() {
    let text_arg_name = "text";
    let omit_newline_arg_name = "omit_newline";

    let matches = App::new("Echor")
        .version("0.1.0")
        .author("Nicolás Rodríguez <nicrodriguezval@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name(text_arg_name)
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name(omit_newline_arg_name)
                .short("n")
                .help("Omit trailing newline")
                .takes_value(false)
        )
        .get_matches();

    let text = matches.values_of_lossy(text_arg_name).unwrap().join(" ");
    let ending = if matches.is_present(omit_newline_arg_name) { "" } else { "\n" };

    print!("{}{}", text, ending);
}
