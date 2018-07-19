

<a id='type__r7rs__exact-integer-negative-or-zero'></a>

# `exact-integer-negative-or-zero` -- `r7rs` Type


<a id='type__r7rs__exact-integer-negative-or-zero__sub-types-tree'></a>

#### Sub-types tree

* **[`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero)**:
  * **[`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero)**;
* **[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative)**;


<a id='type__r7rs__exact-integer-negative-or-zero__super-types'></a>

#### Super-types

 * [`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero);
 * [`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer);


<a id='type__r7rs__exact-integer-negative-or-zero__super-types-recursive'></a>

##### Super-types recursive

 * [`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero);
 * [`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf);
 * [`number-negative-or-zero-not-inf`](../../r7rs/types/number-negative-or-zero-not-inf.md#type__r7rs__number-negative-or-zero-not-inf);
 * [`number-negative-or-zero`](../../r7rs/types/number-negative-or-zero.md#type__r7rs__number-negative-or-zero);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
 * [`real-negative-or-zero`](../../r7rs/types/real-negative-or-zero.md#type__r7rs__real-negative-or-zero);
 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf);
 * [`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf);
 * [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
 * [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
 * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);
 * [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * [`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational);
 * [`exact-real`](../../r7rs/types/exact-real.md#type__r7rs__exact-real);
 * [`exact-complex`](../../r7rs/types/exact-complex.md#type__r7rs__exact-complex);
 * [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);


<a id='type__r7rs__exact-integer-negative-or-zero__sub-types'></a>

#### Sub-types

 * [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
 * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);


<a id='type__r7rs__exact-integer-negative-or-zero__sub-types-recursive'></a>

##### Sub-types recursive

 * [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);


<a id='type__r7rs__exact-integer-negative-or-zero__predicate'></a>

#### Predicate

````
(lambda (value) (and (exact-integer? value) (or (negative? value) (zero? value))))
````


<a id='type__r7rs__exact-integer-negative-or-zero__categories'></a>

#### Categories

 * [`types-numbers`](../../r7rs/categories/types-numbers.md#category__r7rs__types-numbers);


<a id='type__r7rs__exact-integer-negative-or-zero__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

