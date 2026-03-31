# TermLang
Termlang is a language that can only access a 3D memory represented in the terminal by a grid of pixel each having 3 values (r, g, b). All numbers are u8

## Kernel Language
```
<p> ::= [ proc:<name>(<x>,<x>,...){[<s>]} ]
<s> ::= if(<v>){[<s>]}else{[<s>]}  |  while(<v>){[<s>]}  |  <name>(<v>,<v>,...)
<v> ::= <pure number> | '['<v>,<v>,<v>']' | <x>
```
(procedure main is called when program is ran)

## Syntaxic sugars
- Operations : + - * / % == > < >= <= ! & | ^ >> <<  ::=  builtin functions (a+b*c is not valid as it requires an extra memory cell, it must be done in two operations)
- Memory write : [<v>,<v>,<v>] = <v>   ::=   [<v>,<v>,<v>] = $write(<v>)  (later turned into a proc call)
- Defining variables : define x [0,0,0]    |    define x
- Stack : #stack x1 y1 x2 y2   (allocates part of the memory to automatically assign defines that didn't specify the address, x2 and y2 are included)
- Getting variable address (to pass reference as argument) : #x ::= 0,0,0  (also works for pointers, passes pointer reference (2 bytes) as argument)
- Getting address as argument (in function signature) : #arg ::= arg1,arg2,arg3    &    #arg is replaced in the core of the function by arg1,arg2,arg3 (doesn't work for pointers, for pointers specifying two arguments is required)
- Pointers : [0,0] ::= [0,0,0],[0,0,1],[0,0,2]   (to pass as argument, #arg will work) (can be stack allocated with define* ptr, allocated at the end of the stack)
- Functions : x = func(arg1) ::= func(arg1,x1,x2,x3)  &  fn func(arg1) ::= proc func(arg1,$1,$2,$3)  &  return:`<v>` ::= [$1,$2,$3] = `<v>`
- if without else and else if