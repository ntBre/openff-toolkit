//! Code/GraphMol/SmilesParse/SmilesParse.cpp

struct RWMol;

impl RWMol {
    /// omitting SmartsParserParams argument
    fn from_smarts(smarts: &str) -> Self {
        preprocess_smiles();
        RWMol
    }
}

fn preprocess_smiles() {
    todo!()
}
