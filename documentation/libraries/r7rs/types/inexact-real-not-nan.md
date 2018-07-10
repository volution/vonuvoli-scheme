

<a id='type__r7rs__inexact-real-not-nan'></a>

# `inexact-real-not-nan` -- `r7rs` Types


#### Sub-types tree

* **[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf)**;
* **[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan)**:
  * **[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational)**:
    * **[`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer)**;


#### Super-type

[`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan);
[`inexact-real`](../../r7rs/types/inexact-real.md#type__r7rs__inexact-real);
[`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);


##### Super-types recursive

[`inexact-number-not-nan`](../../r7rs/types/inexact-number-not-nan.md#type__r7rs__inexact-number-not-nan);
[`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
[`inexact-complex`](../../r7rs/types/inexact-complex.md#type__r7rs__inexact-complex);
[`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
[`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
[`real`](../../r7rs/types/real.md#type__r7rs__real);


#### Sub-types

[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan);


##### Sub-types recursive

[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);
[`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);


#### Predicate

```
(|lambda| (|value|) (|and| (|real?| |value|) (|inexact?| |value|) (|not| (|nan?| |value|))))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

