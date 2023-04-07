use std::collections::HashMap;


use crate::graph::graphparser::characterjson::AutomatonJSON;

use crate::graph::automatonstate::AutomatonState;


#[derive(Debug)]
pub struct Automaton {
    state: String,
    statemap: HashMap<String, AutomatonState>,
}

impl Automaton {
    pub fn create_automaton(automatonjson: AutomatonJSON) -> Automaton {

        let mut statemap = HashMap::with_capacity(automatonjson.states.len());
        for (statename, statejson) in automatonjson.states.into_iter() {
            statemap.insert(statename.to_owned(), AutomatonState::create_state(statejson));
        }

        return Automaton {
            state: automatonjson.initstate,
            statemap: statemap,
        };
    }
}