use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
pub enum Symbol {
    AugmentingNonTerminal,
    NonTerminal(String),
    EndOfInputTerminal,
    Terminal(String),
}

impl Symbol {

    #[inline]
    pub fn is_terminal(&self) -> bool {
        match self {
            Symbol::EndOfInputTerminal => true,
            Symbol::Terminal(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_nonterminal(&self) -> bool {
        !self.is_terminal()
    }

}

impl Display for Symbol {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        match self {
            Symbol::AugmentingNonTerminal => write!(f, "'"),
            Symbol::NonTerminal(nt) => write!(f, "{}", nt),
            Symbol::EndOfInputTerminal => write!(f, "$"),
            Symbol::Terminal(t) => write!(f, "{}", t)
        }

    }

}

pub struct Grammar {
    start_symbol: Rc<Symbol>,
    symbol_pool: HashMap<String, Rc<Symbol>>,
    productions: HashMap<Rc<Symbol>, Vec<ProductionBody>>
}

pub type ProductionBody = Rc<Vec<Rc<Symbol>>>;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct ProductionReference(pub(crate) Rc<Symbol>, pub(crate) ProductionBody);

impl Display for ProductionReference {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "{} -> ", &*self.0)?;

        let mut it = (&*self.1).iter();

        if let Some(mut s) = it.next() {

            loop {

                write!(f, "{}", s)?;

                match it.next() {
                    None => break,
                    Some(new_s) => {
                        s = new_s;
                        write!(f, " ")?;
                    }
                }

            }

        }

        Ok(())

    }

}

fn string_to_symbol(grammar: &mut Grammar, symbol: String) -> Rc<Symbol> {

    let is_nonterminal = symbol.chars().next().unwrap().is_ascii_uppercase();

    grammar.resolve_symbol(symbol, is_nonterminal)

}

impl Grammar {

    pub fn new(start_symbol: String) -> Self {

        let start_symbol_rc = Rc::new(Symbol::NonTerminal(start_symbol.clone()));

        let mut symbol_pool = HashMap::new();

        symbol_pool.insert(start_symbol, Rc::clone(&start_symbol_rc));

        Self {
            start_symbol: start_symbol_rc,
            symbol_pool,
            productions: HashMap::new()
        }

    }

    fn resolve_symbol(&mut self, symbol: String, non_terminal: bool) -> Rc<Symbol> {

        let symbol_rc = self.symbol_pool
            .entry(symbol.clone())
            .or_insert_with(|| {

                let symbol_rc = Rc::new(if non_terminal {
                    Symbol::NonTerminal(symbol)
                } else {
                    Symbol::Terminal(symbol)
                });

                symbol_rc
            });

        Rc::clone(symbol_rc)

    }

    fn raw_symbols_to_production_body(&mut self, body: Vec<String>) -> Vec<Rc<Symbol>> {

        body.into_iter().map(|entity| {
            string_to_symbol(self, entity)
        }).collect()

    }

    pub fn add_production(&mut self, label: String, body: Vec<String>) {

        let label_rc = self.resolve_symbol(label, true);

        let body = Rc::new(self.raw_symbols_to_production_body(body));

        let productions = self.productions
            .entry(label_rc)
            .or_insert_with(|| vec![]);

        productions.push(body);

    }

    pub fn is_starting_symbol(&self, symbol: &Rc<Symbol>) -> bool {
        Rc::ptr_eq(&self.start_symbol, symbol)
    }

    pub fn productions_for(&self, symbol: &Rc<Symbol>) -> impl Iterator<Item=&ProductionBody> {

        debug_assert!(symbol.is_nonterminal());

        self.productions.get(symbol).unwrap().iter()

    }

    pub fn all_productions(&self) -> impl Iterator<Item=ProductionReference> + '_ {

        self.productions
            .iter()
            .flat_map(|(symbol_rc, production_bodies)| {
                production_bodies.iter().map(move |body| {
                    ProductionReference(Rc::clone(symbol_rc), Rc::clone(body))
                })
            })

    }

}