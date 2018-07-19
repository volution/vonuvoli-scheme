

<a id='type__r7rs__byte-ascii'></a>

# `byte-ascii` -- `r7rs` Type


<a id='type__r7rs__byte-ascii__super-types'></a>

#### Super-types

 * [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);


<a id='type__r7rs__byte-ascii__super-types-recursive'></a>

##### Super-types recursive

 * [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
 * [`integer-positive-or-zero`](../../r7rs/types/integer-positive-or-zero.md#type__r7rs__integer-positive-or-zero);
 * [`rational-positive-or-zero`](../../r7rs/types/rational-positive-or-zero.md#type__r7rs__rational-positive-or-zero);
 * [`real-positive-or-zero-not-inf`](../../r7rs/types/real-positive-or-zero-not-inf.md#type__r7rs__real-positive-or-zero-not-inf);
 * [`number-positive-or-zero-not-inf`](../../r7rs/types/number-positive-or-zero-not-inf.md#type__r7rs__number-positive-or-zero-not-inf);
 * [`number-positive-or-zero`](../../r7rs/types/number-positive-or-zero.md#type__r7rs__number-positive-or-zero);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
 * [`real-positive-or-zero`](../../r7rs/types/real-positive-or-zero.md#type__r7rs__real-positive-or-zero);
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
 * [`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer);
 * [`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational);
 * [`exact-real`](../../r7rs/types/exact-real.md#type__r7rs__exact-real);
 * [`exact-complex`](../../r7rs/types/exact-complex.md#type__r7rs__exact-complex);
 * [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);


<a id='type__r7rs__byte-ascii__sub-types'></a>

#### Sub-types

 * [`code-point-ascii`](../../r7rs/types/code-point-ascii.md#type__r7rs__code-point-ascii);


<a id='type__r7rs__byte-ascii__predicate'></a>

#### Predicate

````
(lambda (value) (and (exact-integer? value) (<= 0 value +127)))
````


<a id='type__r7rs__byte-ascii__categories'></a>

#### Categories

 * [`types-miscellaneous`](../../r7rs/categories/types-miscellaneous.md#category__r7rs__types-miscellaneous);


<a id='type__r7rs__byte-ascii__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

