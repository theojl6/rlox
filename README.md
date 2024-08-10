# Rlox Features
Try it out on the online <a href="https://theojl6.github.io" target="_blank">playground!</a>

## Syntax

### Variables
```
var x = 1;
```

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

# Rust Development
Based on https://craftinginterpreters.com/ 

## Testing
`cargo test`
includes unit tests as well as integration tests, .txt sample files are inside tests/samples/
