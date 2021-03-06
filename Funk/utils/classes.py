
from enum import Enum

KEYWORDS = [
  'println', 'funk', 'if', 'else', 
  'elif', 'while', 'each', 'true',
  'false', 'import', 'null'
]

OPS = [
  '+', '-', '/', '*', '=', 
  '==', '!=', '>', '<', '>=',
  '<=', ':=', '&&', '||', '&',
  '|', '++', '--', '.', '%',
  '^', '!', '+=', '-='
]

class TokenType(Enum):
  Num = 0
  Dec = 1
  LPar = 2
  RPar = 3
  LCurl = 4
  RCurl = 5
  String = 6
  Newline = 7
  Semi = 8
  Comma = 9
  Variable = 10
  Keyword = 11
  Operator = 12
  LBrac = 13
  RBrac = 14

class Token:
  def __init__(self, typ: TokenType, value, position, line):
    self.type, self.value, self.position, self.line = typ, value, position, line
  def __str__(self):
    return f"{{{self.type}, {self.value}}}"
  __repr__ = __str__

class Integer:
  def __init__(self, value):
    self.value = value

  def __str__(self):
    return f"{self.value}"

  __repr__ = __str__

class Decimal:
  def __init__(self, value):
    self.value = value

  def __str__(self):
    return f"{self.value}"

  __repr__ = __str__

class String:
  def __init__(self, value):
    self.value = value

  def __str__(self):
    return f"{self.value}"

  __repr__ = __str__

class Variable:
  def __init__(self, name, scope='global'):
    self.name, self.scope = name, scope

  def __str__(self):
    return f"{{{self.name}, {self.scope}}}"

  __repr__ = __str__

class Container:
  def __init__(self, items):
    self.items = items

  def __str__(self):
    return f"[{', '.join(self.items)}]"

  __repr__ = __str__

class Assignment:
  def __init__(self, typ, variable, value):
    self.type, self.variable, self.value = typ, variable, value

  def __str__(self):
    return f"<{self.type}, {self.variable}, {self.value}>"

  __repr__ = __str__

class BinaryOperator:
  def __init__(self, operator, left, right):
    self.operator, self.left, self.right = operator, left, right

  def __str__(self):
    return f"<BinaryOperator: {self.left} {self.operator} {self.right}>"

  __repr__ = __str__

class UnaryOperator:
  def __init__(self, operator, value):
    self.operator, self.value = operator, value

  def __str__(self):
    return f"<{self.operator}{self.value}>"

  __repr__ = __str__

class Function:
  def __init__(self, name, params, body):
    self.name, self.params, self.body = name, params, body

  def __str__(self):
    return f"Function: {{{self.name}, {self.params}, {self.body}}}"

  __repr__ = __str__

class FunctionParam:
  def __init__(self, name, typ, default):
    self.name, self.type, self.default = name, typ, default

  def __str__(self):
    return f"Param: {{{self.type}, {self.name}, {self.default}}}>"

  __repr__ = __str__

class CallExp:
  def __init__(self, name, args):
    self.name, self.args, = name, args

  def __str__(self):
    return f"Call: {{{self.name}, {self.args}}}"

  __repr__ = __str__

class Condition:
  def __init__(self, typ, exp, body, other):
    self.type, self.exp, self.body, self.other = typ, exp, body, other

  def __str__(self):
    return f"Condition: {{{self.exp}, {self.body}, {self.other}}}"

  __repr__ = __str__

class WhileLoop:
  def __init__(self, exp, body, other):
    self.exp, self.body, self.other = exp, body, other

  def __str__(self):
    return f"WhileLoop: {{{self.exp}, {self.body}, {self.other}}}"

  __repr__ = __str__
