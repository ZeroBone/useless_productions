use crate::grammar::Grammar;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::iter::FromIterator;
use crate::useless::find_useless_productions;

fn test_non_productive(grammar: Grammar, expected_non_productive: &[&str]) {

    let mut non_productive = find_useless_productions(&grammar);

    println!("Non-productive productions: {:#?}", non_productive);

    let non_productive = non_productive
        .into_iter()
        .map(|pr| pr.to_string()).collect::<HashSet<_>>();

    let expected_non_productive =
        HashSet::from_iter(
            expected_non_productive
                .iter()
                .map(|s| {
                    String::from(*s)
                })
        );

    assert_eq!(expected_non_productive, non_productive);

}

#[test]
fn productive_grammar_with_cycles() {

    let mut grammar = Grammar::new("S".to_string());

    grammar.add_production("S".to_string(), vec![
        "C".to_string()
    ]);

    grammar.add_production("S".to_string(), vec![
        "H".to_string()
    ]);

    grammar.add_production("S".to_string(), vec![
        "X".to_string(),
        "E".to_string(),
        "G".to_string(),
        "b".to_string()
    ]);

    grammar.add_production("S".to_string(), vec![
        "X".to_string(),
        "E".to_string()
    ]);

    grammar.add_production("C".to_string(), vec![
        "D".to_string()
    ]);

    grammar.add_production("D".to_string(), vec![
        "a".to_string(),
        "S".to_string(),
        "b".to_string()
    ]);

    grammar.add_production("D".to_string(), vec![
        "s".to_string()
    ]);

    grammar.add_production("D".to_string(), vec![]);

    grammar.add_production("D".to_string(), vec![
        "a".to_string(),
        "F".to_string()
    ]);

    grammar.add_production("H".to_string(), vec![
        "H".to_string()
    ]);

    grammar.add_production("H".to_string(), vec![
        "b".to_string(),
        "F".to_string()
    ]);

    grammar.add_production("F".to_string(), vec![
        "F".to_string(),
        "a".to_string()
    ]);

    grammar.add_production("E".to_string(), vec![
        "a".to_string(),
        "b".to_string()
    ]);

    grammar.add_production("E".to_string(), vec![
        "G".to_string()
    ]);

    grammar.add_production("G".to_string(), vec![
        "a".to_string(),
        "G".to_string()
    ]);

    grammar.add_production("X".to_string(), vec![
        "b".to_string()
    ]);

    grammar.add_production("X".to_string(), vec![
        "a".to_string()
    ]);

    grammar.add_production("X".to_string(), vec![
        "Y".to_string()
    ]);

    grammar.add_production("Y".to_string(), vec![
        "a".to_string()
    ]);

    grammar.add_production("Y".to_string(), vec![
        "X".to_string()
    ]);

    test_non_productive(grammar, &[
        "s -> h",
        "d -> Af",
        "h -> Bf",
        "s -> xegB",
        "h -> h",
        "f -> fA",
        "g -> Ag",
        "e -> g"
    ]);

}