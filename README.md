## Propositional Logic

```
Var       := φ | ψ | χ
Lfn0      := ⊤ | ⊥
Lfn1      := ¬
Lfn2      := ∧ | ∨ | → | ↔
Lfn0Apply := <Lfn0>()
Lfn1Apply := <Lfn1>(<Form>)
Lfn2Apply := <Lfn2>(<Form>,<Form>)
Form      := <Var> | <Lfn0Apply> | <Lfn1Apply> | <Lfn2Apply>
```

```
Zero    := 0
NotZero := 1|2|3|4|5|6|7|8|9
NumAlp  := <Zero> | <NotZero>

Nat     := <NotZero><NumAlp>*

Term    := <Nat>
```
