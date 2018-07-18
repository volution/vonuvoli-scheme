

<a id='type__r7rs__integer-not-zero'></a>

# `integer-not-zero` -- `r7rs` Type


<a id='type__r7rs__integer-not-zero__sub-types-tree'></a>

#### Sub-types tree

* **[`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero)**:
  * **[`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive)**;
  * **[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative)**;
  * **[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero)**;
* **[`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive)**:
  * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
* **[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative)**:
  * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);


<a id='type__r7rs__integer-not-zero__super-types'></a>

#### Super-types

 * [`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero);
 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);


<a id='type__r7rs__integer-not-zero__super-types-recursive'></a>

##### Super-types recursive

 * [`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero);
 * [`complex-not-zero`](../../r7rs/types/complex-not-zero.md#type__r7rs__complex-not-zero);
 * [`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
 * [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
 * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);
 * [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
 * [`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf);
 * [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * [`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf);
 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);


<a id='type__r7rs__integer-not-zero__sub-types'></a>

#### Sub-types

 * [`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero);
 * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);
 * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);


<a id='type__r7rs__integer-not-zero__sub-types-recursive'></a>

##### Sub-types recursive

 * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
 * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
 * [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);


<a id='type__r7rs__integer-not-zero__predicate'></a>

#### Predicate

````
(lambda (value) (and (integer? value) (not (zero? value))))
````


<a id='type__r7rs__integer-not-zero__categories'></a>

#### Categories

 * [`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);


<a id='type__r7rs__integer-not-zero__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

