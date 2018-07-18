

<a id='type__r7rs__number-positive-not-inf'></a>

# `number-positive-not-inf` -- `r7rs` Type


<a id='type__r7rs__number-positive-not-inf__sub-types-tree'></a>

#### Sub-types tree

* **[`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf)**:
  * **[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive)**:
    * **[`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive)**:
      * ...


<a id='type__r7rs__number-positive-not-inf__super-types'></a>

#### Super-types

 * [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
 * [`number-positive-or-zero-not-inf`](../../r7rs/types/number-positive-or-zero-not-inf.md#type__r7rs__number-positive-or-zero-not-inf);


<a id='type__r7rs__number-positive-not-inf__super-types-recursive'></a>

##### Super-types recursive

 * [`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan);
 * [`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number-positive-or-zero`](../../r7rs/types/number-positive-or-zero.md#type__r7rs__number-positive-or-zero);
 * [`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);


<a id='type__r7rs__number-positive-not-inf__sub-types'></a>

#### Sub-types

 * [`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf);


<a id='type__r7rs__number-positive-not-inf__sub-types-recursive'></a>

##### Sub-types recursive

 * [`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive);
 * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);
 * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);


<a id='type__r7rs__number-positive-not-inf__predicate'></a>

#### Predicate

````
(lambda (value) (and (number? value) (positive? value) (not (infinite? value))))
````


<a id='type__r7rs__number-positive-not-inf__categories'></a>

#### Categories

 * [`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);


<a id='type__r7rs__number-positive-not-inf__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

