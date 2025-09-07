use pest::iterators::Pair;
use std::collections::HashMap;

use crate::code::parse::Rule;

pub fn process_labels(labels: &mut HashMap<String, usize>, pairs: pest::iterators::Pair<'_, Rule>) {
    let label_locations: Vec<(usize, Pair<Rule>)> = pairs
        .into_inner()
        .flatten()
        .enumerate()
        .filter(|(_, p)| p.as_rule() == Rule::label)
        .collect();
    for (idx, l) in label_locations {
        labels.insert(l.into_inner().as_str().into(), idx - (2 * labels.len()));
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::code::parse::InputParser;

    use super::*;
    #[test]
    fn label_test() {
        let test_string = "l1: push 1\nl2: push 2\nl3: add\nl4: ret\n";
        let mut labels: HashMap<String, usize> = HashMap::new();
        process_labels(
            &mut labels,
            InputParser::parse(Rule::file, test_string)
                .unwrap()
                .next()
                .unwrap(),
        );
        assert_eq!(*labels.get("l1").unwrap(), 0);
        assert_eq!(*labels.get("l2").unwrap(), 2);
        assert_eq!(*labels.get("l3").unwrap(), 4);
        assert_eq!(*labels.get("l4").unwrap(), 5);
    }
}
