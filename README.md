# Rlox Features

## Syntax

### Variables
`var x = 1;`

### Functions
```
fun Foo(x) { 
    print x;
}
```

### Classes
```
class Bar {
  boo() {
    print 2 + 2;
  }
}
```

### Inheritance
```
class Chocolate < Bar { }
Chocolate().boo()
```

# Development
Based on https://craftinginterpreters.com/ 

Implemented using Rust