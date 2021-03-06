

<a id='type__r7rs__complex-not-zero'></a>

# `complex-not-zero` -- `r7rs` Type


<a id='type__r7rs__complex-not-zero__sub-types-tree'></a>

#### Sub-types tree

* **[`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero)**:
  * **[`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero)**:
    * **[`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero)**:
      * ...
    * **[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive)**:
      * ...
    * **[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative)**:
      * ...
  * **[`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan)**:
    * **[`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive)**:
      * ...
    * **[`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative)**:
      * ...
* **[`complex-not-zero-not-nan`](../../r7rs/types/complex-not-zero-not-nan.md#type__r7rs__complex-not-zero-not-nan)**:
  * [`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan):
    * [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive):
      * ...
    * [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative):
      * ...


<a id='type__r7rs__complex-not-zero__super-types'></a>

#### Super-types

 * [`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
 * [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);


<a id='type__r7rs__complex-not-zero__super-types-recursive'></a>

##### Super-types recursive

 * [`number`](../../r7rs/types/number.md#type__r7rs__number);


<a id='type__r7rs__complex-not-zero__sub-types'></a>

#### Sub-types

 * [`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero);
 * [`complex-not-zero-not-nan`](../../r7rs/types/complex-not-zero-not-nan.md#type__r7rs__complex-not-zero-not-nan);


<a id='type__r7rs__complex-not-zero__sub-types-recursive'></a>

##### Sub-types recursive

 * [`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero);
 * [`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero);
 * [`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero);
 * [`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan);
 * [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
 * [`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf);
 * [`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive);
 * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);
 * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
 * [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
 * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf);
 * [`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);
 * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
 * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
 * [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);


<a id='type__r7rs__complex-not-zero__predicate'></a>

#### Predicate

````
(lambda (value) (and (complex? value) (not (zero? value))))
````


<a id='type__r7rs__complex-not-zero__categories'></a>

#### Categories

 * [`types-numbers`](../../r7rs/categories/types-numbers.md#category__r7rs__types-numbers);


<a id='type__r7rs__complex-not-zero__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

