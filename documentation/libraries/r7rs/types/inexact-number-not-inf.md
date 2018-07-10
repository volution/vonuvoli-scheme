

<a id='type__r7rs__inexact-number-not-inf'></a>

# `inexact-number-not-inf` -- `r7rs` Types


#### Sub-types tree

* **[`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan)**:
  * **[`complex-nan`](../../r7rs/types/complex-nan.md#type__r7rs__complex-nan)**:
    * **[`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan)**;
* **[`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf)**:
  * [`complex-nan`](../../r7rs/types/complex-nan.md#type__r7rs__complex-nan):
    * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
  * **[`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf)**:
    * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
    * **[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan)**:
      * ...
  * **[`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan)**:
    * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan):
      * ...
* **[`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan)**:
  * [`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan):
    * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan):
      * ...


#### Super-type

[`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
[`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);


##### Super-types recursive

[`number`](../../r7rs/types/number.md#type__r7rs__number);


#### Sub-types

[`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
[`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf);
[`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan);


##### Sub-types recursive

[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);
[`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);
[`complex-nan`](../../r7rs/types/complex-nan.md#type__r7rs__complex-nan);
[`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
[`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf);
[`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan);
[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan);


#### Predicate

```
(|lambda| (|value|) (|and| (|number?| |value|) (|inexact?| |value|) (|not| (|infinite?| |value|))))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

