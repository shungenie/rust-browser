use alloc::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node{
    ExpressionStatement(Option<Rc<Node>>),
    AdditiveExpression {
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    },
    AssignmentExpression {
        operator: char,
        left: Option<Rc<Node>>,
        right: Option<Rc<Node>>,
    },
    MemberExpression {
        object: Option<Rc<Node>>,
        property: Option<Rc<Node>>,
    },
    NumericLiteral(u64),
}

impl Node {
    pub fn new_expression_statement(expression: Option<Rc<Self>>) -> Option<Rc<Self>> {
        Some(Rc::new(Node::ExpressionStatement(expression)))
    }
}