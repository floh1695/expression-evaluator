/* Note: This feels way too complex, probably gonna scrap it */

// export type AstNode = ValueNode;

// export type ValueNode = ConstantNode | OperatorNode | ;

// type ValueNodeTypes = 'constant' | 'operator' | 'expression';
// type BaseValueNode<A extends ValueNodeTypes> = {
//   nodeType: A,
//   eval: () => number
// };

// type ConstantNode = BaseValueNode<'constant'> & {
//   value: number
// };

// type OperatorNode = BaseValueNode<'operator'> & {
//   operator: string,
//   operands: ValueNode[]
// };

// type ExpressionNode = BaseValueNode<'expression'> & {

// };
