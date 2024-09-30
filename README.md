# Rust Parsing

## Recursive Descent Parsing of Boolean Formulas
Adapting https://vey.ie/2018/10/04/RecursiveDescent.html from Python into Rust.

```python
from rust_parsing import parse_arithmetic

parse_arithmetic("!A -> B | A & C")
# ((Not (A)) implies ((B) or ((A) and (C))))

parse_arithmetic("!A -> (B | A) & C")
# ((Not (A)) implies (((B) or (A)) and (C)))
```
