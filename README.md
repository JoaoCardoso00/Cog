# Cog
![image](https://github.com/JoaoCardoso00/Cog/assets/33937520/d7f6bcad-93a1-4960-b5cc-1684af875881)

## Cog is a general purpose, interpreted programming language

Cog has support for the following:

- [x] numeric literals
- [x] arithmetic operations
- [x] variable declaration
- [x] native function calls
- [x] objects and object member calls
- [x] string literals
- [ ] user defined functions
- [ ] conditionals
- [ ] loops
``
Since this is a college project i dont think i will add anything outside the list mentioned above, this is a side project, feel free to use the contents in this repo as you wish.

## Runing the project:

1 -  Clone the repo:
``` bash
git clone https://github.com/JoaoCardoso00/Cog.git
```

2 - Run the program:
``` bash
cargo run -- example.cog
```
you can write in the example.cog or create a new one and pass the path to the file as the argument for the program

the program also supports an optional -ast flag that prints the generated AST to the standard output

``` bash
cargo run -- example.cog -ast
```

Example:

```
let x = 5;
```

Output:

```bash
  AST {
    kind: "Program",
    statements: [
        ASTStatement {
            kind: VariableDeclaration(
                VariableDeclaration {
                    constant: false,
                    identifier: String(
                        "x",
                    ),
                    value: Some(
                        ASTExpression {
                            kind: NumericLiteral,
                            body: Value(
                                Number(
                                    5.0,
                                ),
                            ),
                        },
                    ),
                },
            ),
        },
    ],
}
```

## Native functions supported:

### print(args)

prints to the standard output all the arguments it was provided

#### examples:

```
let x = 10;

print(5 + 5)
print(x / 2)
```

it also supports object fields as arguments

```
let obj = {
  field: {
    value: 20
  }
}

print(obj.field.value)
```
