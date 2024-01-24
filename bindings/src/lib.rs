use dandy::dfa::Dfa;
use dandy::dfa::parse::DfaParseError;
use dandy::nfa::Nfa;
use dandy::nfa::parse::NfaParseError;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn check_dfa(input: &str) -> Result<(), String> {
    let _dfa: Dfa = dandy::parser::dfa(input)
        .map_err(|e| format!("Error parsing DFA: {e:?}"))?
        .try_into()
        .map_err(|e: DfaParseError| e.to_string())?;
    Ok(())
}

#[wasm_bindgen]
pub fn check_nfa(input: &str) -> Result<(), String> {
    let _nfa: Nfa = dandy::parser::nfa(input)
        .map_err(|e| format!("Error parsing NFA: {e:?}"))?
        .try_into()
        .map_err(|e: NfaParseError| e.to_string())?;
    Ok(())
}
