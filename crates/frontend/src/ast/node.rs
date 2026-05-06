
pub enum NodeType {
    While(NodeType),

}

pub trait Visitor {
    fn visit(&self, NodeType::While)
}