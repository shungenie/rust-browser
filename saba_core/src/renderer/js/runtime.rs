use crate::renderer::js::ast::Program;
use crate::renderer::js::ast::Node;
use alloc::rc::Rc;
use core::borrow::Borrow;
use core::ops::Add;
use core::ops::Sub;

#[derive(Debug, Clone)]
pub struct JsRuntime{}

impl JsRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&mut self, program: &Program) {
        for node in program.body() {
            self.eval(&Some(node.clone()));
        }
    }
    
    fn eval(
        &mut self,
        node: &Option<Rc<Node>>,
    ) -> Option<RuntimeValue> {
        let node = match node {
            Some(n) => n,
            None => return None,
        };

        match node.borrow() {
            Node::ExpressionStatement(expr) => return self.eval(&expr),
            Node::AdditiveExpression { 
                operator, 
                left, 
                right 
            } => {
                let left_value = match self.eval(&left) {
                    Some(value) => value,
                    None => return None,
                };
                let right_value = match self.eval(&right) {
                    Some(value) => value,
                    None => return None,
                };

                if operator == &'+' {
                    Some(left_value + right_value)
                } else if operator == &'-' {
                    Some(left_value - right_value)
                } else {
                    None
                }
            }
            Node::AssignmentExpression {
                operator: _,
                left: _,
                right: _,
            } => {
                //todo
                None
            }
            Node::MemberExpression { 
                object: _, 
                property: _,
            } => {
                //todo
                None
            }
            Node::NumericLiteral(value) => Some(RuntimeValue::Number(*value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number(u64),
}

impl Add<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn add(self, rhs: RuntimeValue) -> RuntimeValue {
        let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (&self, &rhs);
        return RuntimeValue::Number(left_num + right_num)
    }
}

impl Sub<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn sub(self, rhs: RuntimeValue) -> RuntimeValue {
        let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (&self, &rhs);
        return RuntimeValue::Number(left_num - right_num);
    }
}