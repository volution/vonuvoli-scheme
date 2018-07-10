

<a id='type__r7rs__inexact-complex'></a>

# `inexact-complex` -- `r7rs` Types


#### Sub-types tree

* **[`inexact-real`](../../r7rs/types/inexact-real.md#type__r7rs__inexact-real)**:
  * **[`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf)**:
    * **[`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan)**;
    * **[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan)**:
      * ...
  * **[`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan)**:
    * **[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf)**;
    * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan):
      * ...
* **[`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf)**:
  * **[`complex-nan`](../../r7rs/types/complex-nan.md#type__r7rs__complex-nan)**:
    * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
  * [`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf):
    * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
    * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan):
      * ...
  * **[`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan)**:
    * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan):
      * ...
* **[`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan)**:
  * **[`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf)**:
    * [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
  * [`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan):
    * [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
    * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan):
      * ...
  * [`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan):
    * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan):
      * ...


#### Super-type

[`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
[`complex`](../../r7rs/types/complex.md#type__r7rs__complex);


##### Super-types recursive

[`number`](../../r7rs/types/number.md#type__r7rs__number);


#### Sub-types

[`inexact-real`](../../r7rs/types/inexact-real.md#type__r7rs__inexact-real);
[`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf);
[`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan);


##### Sub-types recursive

[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);
[`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);
[`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf);
[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
[`complex-nan`](../../r7rs/types/complex-nan.md#type__r7rs__complex-nan);
[`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
[`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf);
[`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan);
[`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan);
[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan);


#### Predicate

```
(|lambda| (|value|) (|and| (|complex?| |value|) (|inexact?| |value|)))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

