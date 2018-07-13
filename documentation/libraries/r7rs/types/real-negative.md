

<a id='type__r7rs__real-negative'></a>

# `real-negative` -- `r7rs` Types


<a id='type__r7rs__real-negative__sub-types-tree'></a>

#### Sub-types tree

* **[`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf)**:
  * **[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative)**:
    * **[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative)**:
      * ...


<a id='type__r7rs__real-negative__super-types'></a>

#### Super-types

 * [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
 * [`real-negative-or-zero`](../../r7rs/types/real-negative-or-zero.md#type__r7rs__real-negative-or-zero);
 * [`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan);


<a id='type__r7rs__real-negative__super-types-recursive'></a>

##### Super-types recursive

 * [`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan);
 * [`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`number-negative-or-zero`](../../r7rs/types/number-negative-or-zero.md#type__r7rs__number-negative-or-zero);
 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`complex-not-zero-not-nan`](../../r7rs/types/complex-not-zero-not-nan.md#type__r7rs__complex-not-zero-not-nan);
 * [`complex-not-zero`](../../r7rs/types/complex-not-zero.md#type__r7rs__complex-not-zero);
 * [`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero);


<a id='type__r7rs__real-negative__sub-types'></a>

#### Sub-types

 * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf);


<a id='type__r7rs__real-negative__sub-types-recursive'></a>

##### Sub-types recursive

 * [`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);
 * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
 * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);


<a id='type__r7rs__real-negative__predicate'></a>

#### Predicate

````
(lambda (value) (and (real? value) (negative? value)))
````


<a id='type__r7rs__real-negative__categories'></a>

#### Categories

 * [`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);


<a id='type__r7rs__real-negative__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

