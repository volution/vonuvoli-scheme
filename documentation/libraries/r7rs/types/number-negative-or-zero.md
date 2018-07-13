

<a id='type__r7rs__number-negative-or-zero'></a>

# `number-negative-or-zero` -- `r7rs` Types


<a id='type__r7rs__number-negative-or-zero__sub-types-tree'></a>

#### Sub-types tree

* **[`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative)**:
  * **[`number-negative-not-inf`](../../r7rs/types/number-negative-not-inf.md#type__r7rs__number-negative-not-inf)**:
    * **[`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf)**:
      * ...
  * **[`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative)**:
    * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf):
      * ...
* **[`number-negative-or-zero-not-inf`](../../r7rs/types/number-negative-or-zero-not-inf.md#type__r7rs__number-negative-or-zero-not-inf)**:
  * **[`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero)**:
    * **[`complex-zero`](../../r7rs/types/complex-zero.md#type__r7rs__complex-zero)**:
      * ...
  * [`number-negative-not-inf`](../../r7rs/types/number-negative-not-inf.md#type__r7rs__number-negative-not-inf):
    * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf):
      * ...
  * **[`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf)**:
    * **[`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero)**:
      * ...
    * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf):
      * ...
    * **[`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero)**:
      * ...
* **[`real-negative-or-zero`](../../r7rs/types/real-negative-or-zero.md#type__r7rs__real-negative-or-zero)**:
  * [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative):
    * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf):
      * ...
  * [`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf):
    * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero):
      * ...
    * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf):
      * ...
    * [`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero):
      * ...


<a id='type__r7rs__number-negative-or-zero__super-types'></a>

#### Super-types

 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);


<a id='type__r7rs__number-negative-or-zero__super-types-recursive'></a>

##### Super-types recursive

 * [`number`](../../r7rs/types/number.md#type__r7rs__number);


<a id='type__r7rs__number-negative-or-zero__sub-types'></a>

#### Sub-types

 * [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
 * [`number-negative-or-zero-not-inf`](../../r7rs/types/number-negative-or-zero-not-inf.md#type__r7rs__number-negative-or-zero-not-inf);
 * [`real-negative-or-zero`](../../r7rs/types/real-negative-or-zero.md#type__r7rs__real-negative-or-zero);


<a id='type__r7rs__number-negative-or-zero__sub-types-recursive'></a>

##### Sub-types recursive

 * [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * [`complex-zero`](../../r7rs/types/complex-zero.md#type__r7rs__complex-zero);
 * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
 * [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero);
 * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero);
 * [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
 * [`number-negative-not-inf`](../../r7rs/types/number-negative-not-inf.md#type__r7rs__number-negative-not-inf);
 * [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
 * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf);
 * [`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);
 * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
 * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
 * [`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf);
 * [`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero);
 * [`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero);
 * [`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero);
 * [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);


<a id='type__r7rs__number-negative-or-zero__predicate'></a>

#### Predicate

````
(lambda (value) (and (number? value) (or (negative? value) (zero? value))))
````


<a id='type__r7rs__number-negative-or-zero__categories'></a>

#### Categories

 * [`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);


<a id='type__r7rs__number-negative-or-zero__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

