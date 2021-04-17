use crate::grammar::{Symbol, Grammar, ProductionReference};
use std::collections::{HashMap, HashSet, BTreeSet};
use std::rc::Rc;

enum Edge {
    Production(ProductionReference),
    CountDown(BTreeSet<Rc<Symbol>>)
}

struct FindUselessProductions {
    graph: HashMap<BTreeSet<Rc<Symbol>>, Vec<Edge>>,
    starting_productions: HashSet<ProductionReference>
}

fn single_nonterminal_node(symbol: &Rc<Symbol>) -> BTreeSet<Rc<Symbol>> {
    let mut set = BTreeSet::new();
    set.insert(Rc::clone(symbol));
    set
}

impl FindUselessProductions {

    fn new() -> Self {
        Self {
            graph: HashMap::new(),
            starting_productions: HashSet::new()
        }
    }

    fn handle_production(&mut self, pr: ProductionReference) {

        let ProductionReference(label, body) = pr;

        let mut right_nonterminals = BTreeSet::new();

        for symbol in &*body {

            let symbol = &*symbol;

            if symbol.is_terminal() {
                continue;
            }

            right_nonterminals.insert(Rc::clone(symbol));

        }

        // create node for the label of the current production if the node is not yet present
        self.graph
            .entry(single_nonterminal_node(&label))
            .or_insert_with(|| vec![]);

        if right_nonterminals.is_empty() {

            // this production contains only terminal symbols on it's right hand sides
            // it is therefore productive

            self.starting_productions.insert(ProductionReference(label, body));

            return;

        }

        if right_nonterminals.len() == 1 {

            let key = single_nonterminal_node(
                right_nonterminals.iter().next().unwrap()
            );

            self.graph
                .entry(key)
                .or_insert_with(|| vec![])
                .push(Edge::Production(ProductionReference(label, body)));

            return;

        }

        debug_assert!(right_nonterminals.len() >= 2);

        for nonterminal in right_nonterminals.iter() {

            debug_assert!(nonterminal.is_nonterminal());

            self.graph
                .entry(single_nonterminal_node(nonterminal))
                .or_insert_with(|| vec![])
                .push(Edge::CountDown(right_nonterminals.clone()));

        }

        self.graph
            .entry(right_nonterminals)
            .or_insert_with(|| vec![])
            .push(Edge::Production(ProductionReference(label, body)));
    }

    fn get_productive_productions(&self) -> HashSet<ProductionReference> {

        let mut stack = self.starting_productions
            .iter()
            .map(|pr| single_nonterminal_node(&pr.0))
            .collect::<Vec<_>>();

        let mut visited = self.starting_productions.clone();

        let mut counters = HashMap::new();

        while let Some(node) = stack.pop() {

            for edge in self.graph.get(&node).unwrap() {

                match edge {
                    Edge::Production(pr) => {

                        if !visited.contains(pr) {

                            visited.insert(pr.clone());

                            stack.push(single_nonterminal_node(&pr.0));

                        }

                    }
                    Edge::CountDown(symbols) => {

                        // countdown edges can only go from nodes containing
                        // single non-terminals
                        debug_assert!(node.len() == 1);
                        // countdown edges can only lead to nodes containing
                        // multiple symbols
                        debug_assert!(symbols.len() >= 2);

                        if !counters.contains_key(&node) {

                            let counter = counters
                                .entry(symbols.clone())
                                .or_insert_with(|| symbols.len());

                            debug_assert!(*counter >= 1);

                            *counter -= 1;

                            if *counter == 0 {
                                // this is the last remaining edge to a compound node
                                // we therefore go over this edge in our dfs

                                stack.push(symbols.clone());
                            }
                        }
                    }
                }

            }

            if node.len() == 1 {
                counters.insert(node.clone(), 0);
            }

        }

        visited
    }

    fn get_non_productive_productions(&self, grammar: &Grammar) -> HashSet<ProductionReference> {

        let productive_productions = self.get_productive_productions();

        let all_productions = grammar
            .all_productions()
            .collect::<HashSet<ProductionReference>>();

        all_productions
            .difference(&productive_productions)
            .map(|e| e.clone())
            .collect::<HashSet<ProductionReference>>()
    }

}

pub fn find_useless_productions(grammar: &Grammar) -> HashSet<ProductionReference> {
    let mut useless = FindUselessProductions::new();
    for pr in grammar.all_productions() {
        useless.handle_production(pr);
    }
    useless.get_non_productive_productions(grammar)
}