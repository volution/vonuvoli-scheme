

<a id='type__r7rs__inexact-number-not-nan'></a>

# `inexact-number-not-nan` -- `r7rs` Types


#### Sub-types tree

* **[`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf)**:
  * **[`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf)**:
    * **[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf)**;
* **[`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan)**:
  * [`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf):
    * [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
  * **[`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan)**:
    * [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
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
[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);


##### Super-types recursive

[`number`](../../r7rs/types/number.md#type__r7rs__number);


#### Sub-types

[`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
[`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan);
[`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan);


##### Sub-types recursive

[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);
[`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);
[`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf);
[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
[`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan);
[`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan);
[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan);


#### Predicate

```
(|lambda| (|value|) (|and| (|number?| |value|) (|inexact?| |value|) (|not| (|nan?| |value|))))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

