use tree_sitter_symbols_rust::NodeType;

pub struct SymbolPath {
    pub segments: Vec<SymbolSegment>,
}

pub struct SymbolSegment {
    pub node_type: NodeType,
    pub name: Option<String>,
}
