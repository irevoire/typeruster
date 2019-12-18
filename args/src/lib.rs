use clap::{App, Arg, SubCommand};
use text::fix::computer::*;
use text::fix::english::*;
use text::fix::french::*;
use text::fix::german::*;
use text::fix::*;
use text::Text;

pub struct Args {
    source: Text,
    fix: Vec<fn(&mut Text)>,
}

impl Args {
    pub fn apply_fix(&mut self) {
        for fix in self.fix.iter() {
            fix(&mut self.source);
        }
    }

    pub fn text(&self) -> &Text {
        &self.source
    }
}

pub fn parse() -> Args {
    let matches = App::new("TypeRuster")
        .author("Thomas C. <thomas.campistron.etu@univ-lille.fr>")
        .about("Typeracer written in rust")
        .arg(
            Arg::with_name("from")
                .help("Let you choose the text you want to type from a file")
                .long("from")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("computer-single-quote")
                .help("Only use computer single quote: '")
                .conflicts_with("french-single-quote")
                .long("computer-single-quote"),
        )
        .arg(
            Arg::with_name("computer-double-quote")
                .help("Only use computer double quote: \"")
                .conflicts_with_all(&[
                    "english-double-quote",
                    "german-double-quote",
                    "french-double-quote",
                ])
                .long("computer-double-quote"),
        )
        .arg(
            Arg::with_name("computer-dash")
                .help("Only use computer dash: -")
                .long("computer-dash"),
        )
        .arg(
            Arg::with_name("english-double-quote")
                .help("Use english double quote: “ and ”")
                .conflicts_with_all(&[
                    "computer-double-quote",
                    "german-double-quote",
                    "french-double-quote",
                ])
                .long("english-double-quote"),
        )
        .arg(
            Arg::with_name("german-double-quote")
                .help("Use german double quote: „ and “")
                .conflicts_with_all(&[
                    "computer-double-quote",
                    "english-double-quote",
                    "french-double-quote",
                ])
                .long("german-double-quote"),
        )
        .arg(
            Arg::with_name("french-single-quote")
                .help("Use french single quote: ’")
                .conflicts_with("computer-single-quote")
                .long("french-single-quote"),
        )
        .arg(
            Arg::with_name("french-double-quote")
                .help("Use french double quote: « and »")
                .conflicts_with_all(&[
                    "computer-double-quote",
                    "english-double-quote",
                    "german-double-quote",
                ])
                .long("french-double-quote"),
        )
        .arg(
            Arg::with_name("french-unbreakable-space")
                .help("Only use french double quote: « and »")
                .long("french-unbreakable-space"),
        )
        .arg(
            Arg::with_name("french-ligatures")
                .help("Use french ligatures: æ and œ")
                .long("french-ligatures"),
        )
        .arg(
            Arg::with_name("french-ae-ligature")
                .help("Use french ligatures: æ")
                .long("french-ae-ligature"),
        )
        .arg(
            Arg::with_name("french-oe-ligature")
                .help("Use french ligatures: œ")
                .long("french-oe-ligature"),
        )
        .subcommand(
            SubCommand::with_name("preset")
                .help("Choose a preset between: azerty, bepo, dvorak, qwerty")
                .about("Choose a preset")
                .arg(
                    Arg::with_name("preset")
                        .help("Select a preset")
                        .required(true)
                        .possible_value("azerty")
                        .possible_values(&["bepo", "bépo"])
                        .possible_value("dvorak")
                        .possible_value("qwerty"),
                ),
        )
        .get_matches();
    let mut fix: Vec<fn(&mut Text)> = Vec::new();
    let mut source = text::source::mots_surannes::get_text();

    if let Some(preset) = matches.subcommand_matches("preset") {
        let preset = preset.value_of("preset").unwrap();
        match preset {
            "azerty" => fix.push(Text::azerty_preset),
            "bepo" => fix.push(Text::bepo_french_preset),
            "bépo" => fix.push(Text::bepo_french_preset),
            "dvorak" => fix.push(Text::azerty_preset),
            "qwerty" => fix.push(Text::azerty_preset),
            _ => (),
        }
    }

    if let Some(file) = matches.value_of("from") {
        source = text::source::file::from(file);
    }

    if matches.is_present("computer-single-quote") {
        fix.push(Text::use_computer_quote);
    }

    if matches.is_present("computer-double-quote") {
        fix.push(Text::use_computer_double_quote);
    }

    if matches.is_present("computer-dash") {
        fix.push(Text::use_computer_dash);
    }

    if matches.is_present("english-double-quote") {
        fix.push(Text::use_english_double_quote);
    }

    if matches.is_present("german-double-quote") {
        fix.push(Text::use_german_double_quote);
    }

    if matches.is_present("french-single-quote") {
        fix.push(Text::use_french_quote);
    }

    if matches.is_present("french-double-quote") {
        fix.push(Text::use_french_double_quote);
    }

    if matches.is_present("french-unbreakable-space") {
        fix.push(Text::use_french_unbreakable_space);
    }

    if matches.is_present("french-ligatures") {
        fix.push(Text::use_french_ligatures);
    }

    if matches.is_present("french-ae-ligature") {
        fix.push(Text::use_french_ligature_in_ae);
    }

    if matches.is_present("french-oe-ligature") {
        fix.push(Text::use_french_ligature_in_oe);
    }

    Args { fix, source }
}
