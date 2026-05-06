# Descriptions

## name

[_a-zA-Z][_a-zA-Z0-9]*

## type

[_a-zA-Z][_a-zA-Z0-9]*

### default types

**Signed Integers**:
- i8
- i16
- i32
- i64

**Unsigned Integers**:
- u8
- u16
- u32
- u64

**Floats**:
- f32
- f64

**References**:

Each type have its references counter part by adding a '&' in front of the type name:

References allow to pass a variable to a function so that she can modifie it.

examples:
- f32 => &f32
- u16 => &u16

## Arrays

### declaraton

// type is the type of the elementes in the array
// name will be the name of the created array
// exp should be a int and represent the number of element:

type name '[' exp ']'

### usage

name '[' exp ']'

## Struct

### decleration

// name is the name of the created structure
// type is the type of the attribute
// name1 is the name of the attribute created

'struct' name '{'
    { type name1 ';' } // attribute definitions
'}'

### Usage

- name(to pass a copy)
- &name(to pass a reference)
- name.name1 (return the value within the fields)

## Variable

### declaration

type name [ '=" exp ] ';'

## Les proto de bases

### Les fonctions

#### Declaration

type name '(' { vars ',' } ')' actions

```
fn strlen(string) -> i32 {
	// TODO
}
```

#### utilisations

name '(' { exp ',' } ')'

```
strlen(myStr)
```

### if else

'if' '(' condition ')' actions

```
if (n + 5 == 10) {

}
```

### while (jsp si je m'emmerde avec ca)

'while' '(' condition ')' actions

### for

'for' '(' { dec ',' } ';' condition ';' { ?? ',' } ')' actions

### switch

'switch' '(' exp1 ')' '{'
    { 'case' exp2 => actions ',' }
'}'


### Actions

'{' { exp ';' } '}'