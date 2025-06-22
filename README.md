# Readme

WIP compiler for the [kaleidoscope programming language](https://en.wikipedia.org/wiki/Kaleidoscope_(programming_language)).

## Supported features

### Basic arithmetical operations

1 + (2 - 3) * 4

### Function definitions

def abc(d, e, f) d+e+f;

### Function calls

abc(1,2,3);

## Formal definition

toplevelexpr ::= expr
expr ::= primary binoprhs
primary ::= numberexpr
primary ::= identifierexpr
primary ::= parenthesisexpr
binoprhs ::= (('+'|'-'|'*') primary)*
numberexpr ::= number
identifierexpr ::= identifier
identifierexpr ::= identifier '(' expr* ')'
parenthesisexpr ::= '(' expr ')'

definition ::= 'def' prototype expr
prototype ::= identifier '(' identifier* ')'
external ::= 'extern' prototype
