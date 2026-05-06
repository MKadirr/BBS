# Descriptions

## un nom

## un type

## Arrays

## Struct

## Vardec

## Les proto de bases

### Les fonctions.

#### Declaration

'fn' name '(' { vars ',' } ')' ['->' type] actions

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

'switch' '(' exp ')' actions