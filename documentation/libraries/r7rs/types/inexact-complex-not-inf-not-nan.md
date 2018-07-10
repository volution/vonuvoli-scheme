

<a id='type__r7rs__inexact-complex-not-inf-not-nan'></a>

# `inexact-complex-not-inf-not-nan` -- `r7rs` Types


#### Sub-types tree

* **[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan)**:
  * **[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational)**:
    * **[`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer)**;


#### Super-type

[`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan);
[`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf);
[`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan);
[`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);


##### Super-types recursive

[`inexact-number-not-inf`](../../r7rs/types/inexact-number-not-inf.md#type__r7rs__inexact-number-not-inf);
[`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
[`inexact-number-not-nan`](../../r7rs/types/inexact-number-not-nan.md#type__r7rs__inexact-number-not-nan);
[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
[`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
[`inexact-complex`](../../r7rs/types/inexact-complex.md#type__r7rs__inexact-complex);
[`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
[`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf);
[`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);


#### Sub-types

[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan);


##### Sub-types recursive

[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);
[`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);


#### Predicate

```
(|lambda| (|value|) (|and| (|complex?| |value|) (|inexact?| |value|) (|not| (|infinite?| |value|)) (|not| (|nan?| |value|))))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

