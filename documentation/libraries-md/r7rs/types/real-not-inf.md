

<a id='type__r7rs__real-not-inf'></a>

# `real-not-inf` -- `r7rs` Type


<a id='type__r7rs__real-not-inf__sub-types-tree'></a>

#### Sub-types tree

* **[`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan)**:
  * **[`rational`](../../r7rs/types/rational.md#type__r7rs__rational)**:
    * **[`integer`](../../r7rs/types/integer.md#type__r7rs__integer)**:
      * ...
    * **[`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational)**:
      * ...
    * **[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational)**:
      * ...
    * **[`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero)**:
      * ...
    * **[`rational-positive-or-zero`](../../r7rs/types/rational-positive-or-zero.md#type__r7rs__rational-positive-or-zero)**:
      * ...
    * **[`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero)**:
      * ...
  * **[`exact-real`](../../r7rs/types/exact-real.md#type__r7rs__exact-real)**:
    * [`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational):
      * ...
  * **[`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero)**:
    * **[`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero)**:
      * ...
  * **[`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan)**:
    * [`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational):
      * ...
* **[`real-positive-or-zero-not-inf`](../../r7rs/types/real-positive-or-zero-not-inf.md#type__r7rs__real-positive-or-zero-not-inf)**:
  * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero):
    * [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero):
      * ...
  * **[`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf)**:
    * **[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive)**:
      * ...
  * [`rational-positive-or-zero`](../../r7rs/types/rational-positive-or-zero.md#type__r7rs__rational-positive-or-zero):
    * [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero):
      * ...
    * [`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive):
      * ...
    * **[`integer-positive-or-zero`](../../r7rs/types/integer-positive-or-zero.md#type__r7rs__integer-positive-or-zero)**:
      * ...
* **[`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf)**:
  * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero):
    * [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero):
      * ...
  * **[`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf)**:
    * **[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative)**:
      * ...
  * [`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero):
    * [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero):
      * ...
    * [`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative):
      * ...
    * **[`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero)**:
      * ...
* **[`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf)**:
  * **[`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan)**;
  * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan):
    * [`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational):
      * ...


<a id='type__r7rs__real-not-inf__super-types'></a>

#### Super-types

 * [`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);


<a id='type__r7rs__real-not-inf__super-types-recursive'></a>

##### Super-types recursive

 * [`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);


<a id='type__r7rs__real-not-inf__sub-types'></a>

#### Sub-types

 * [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
 * [`real-positive-or-zero-not-inf`](../../r7rs/types/real-positive-or-zero-not-inf.md#type__r7rs__real-positive-or-zero-not-inf);
 * [`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf);
 * [`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf);


<a id='type__r7rs__real-not-inf__sub-types-recursive'></a>

##### Sub-types recursive

 * [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * [`exact-real`](../../r7rs/types/exact-real.md#type__r7rs__exact-real);
 * [`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational);
 * [`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer);
 * [`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);
 * [`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);
 * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
 * [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero);
 * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero);
 * [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
 * [`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero);
 * [`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero);
 * [`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero);
 * [`integer-even`](../../r7rs/types/integer-even.md#type__r7rs__integer-even);
 * [`integer-odd`](../../r7rs/types/integer-odd.md#type__r7rs__integer-odd);
 * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
 * [`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf);
 * [`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive);
 * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);
 * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
 * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf);
 * [`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);
 * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
 * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
 * [`rational-positive-or-zero`](../../r7rs/types/rational-positive-or-zero.md#type__r7rs__rational-positive-or-zero);
 * [`integer-positive-or-zero`](../../r7rs/types/integer-positive-or-zero.md#type__r7rs__integer-positive-or-zero);
 * [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
 * [`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero);
 * [`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero);
 * [`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero);
 * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan);
 * [`code-point-unicode`](../../r7rs/types/code-point-unicode.md#type__r7rs__code-point-unicode);
 * [`code-point-ascii`](../../r7rs/types/code-point-ascii.md#type__r7rs__code-point-ascii);
 * [`range-value`](../../r7rs/types/range-value.md#type__r7rs__range-value);
 * [`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
 * [`range-length`](../../r7rs/types/range-length.md#type__r7rs__range-length);
 * [`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
 * [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
 * [`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
 * [`byte-ascii`](../../r7rs/types/byte-ascii.md#type__r7rs__byte-ascii);


<a id='type__r7rs__real-not-inf__predicate'></a>

#### Predicate

````
(lambda (value) (and (real? value) (not (infinite? value))))
````


<a id='type__r7rs__real-not-inf__categories'></a>

#### Categories

 * [`types-numbers`](../../r7rs/categories/types-numbers.md#category__r7rs__types-numbers);


<a id='type__r7rs__real-not-inf__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

