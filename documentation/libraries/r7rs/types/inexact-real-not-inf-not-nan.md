

<a id='type__r7rs__inexact-real-not-inf-not-nan'></a>

# `inexact-real-not-inf-not-nan` -- `r7rs` Type


<a id='type__r7rs__inexact-real-not-inf-not-nan__sub-types-tree'></a>

#### Sub-types tree

* **[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational)**:
  * **[`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer)**;


<a id='type__r7rs__inexact-real-not-inf-not-nan__super-types'></a>

#### Super-types

 * [`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan);
 * [`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf);
 * [`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan);
 * [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);


<a id='type__r7rs__inexact-real-not-inf-not-nan__super-types-recursive'></a>

##### Super-types recursive

 * [`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan);
 * [`inexact-number-not-inf`](../../r7rs/types/inexact-number-not-inf.md#type__r7rs__inexact-number-not-inf);
 * [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
 * [`inexact-number-not-nan`](../../r7rs/types/inexact-number-not-nan.md#type__r7rs__inexact-number-not-nan);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
 * [`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf);
 * [`inexact-complex`](../../r7rs/types/inexact-complex.md#type__r7rs__inexact-complex);
 * [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
 * [`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf);
 * [`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan);
 * [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);
 * [`inexact-real`](../../r7rs/types/inexact-real.md#type__r7rs__inexact-real);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf);
 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);


<a id='type__r7rs__inexact-real-not-inf-not-nan__sub-types'></a>

#### Sub-types

 * [`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);


<a id='type__r7rs__inexact-real-not-inf-not-nan__sub-types-recursive'></a>

##### Sub-types recursive

 * [`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);


<a id='type__r7rs__inexact-real-not-inf-not-nan__predicate'></a>

#### Predicate

````
(lambda (value) (and (real? value) (inexact? value) (not (infinite? value)) (not (nan? value))))
````


<a id='type__r7rs__inexact-real-not-inf-not-nan__categories'></a>

#### Categories

 * [`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);


<a id='type__r7rs__inexact-real-not-inf-not-nan__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

