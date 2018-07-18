

<a id='type__r7rs__number'></a>

# `number` -- `r7rs` Type


<a id='type__r7rs__number__sub-types-tree'></a>

#### Sub-types tree

* **[`complex`](../../r7rs/types/complex.md#type__r7rs__complex)**:
  * **[`real`](../../r7rs/types/real.md#type__r7rs__real)**:
    * **[`inexact-real`](../../r7rs/types/inexact-real.md#type__r7rs__inexact-real)**:
      * ...
    * **[`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero)**:
      * ...
    * **[`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf)**:
      * ...
    * **[`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan)**:
      * ...
  * **[`inexact-complex`](../../r7rs/types/inexact-complex.md#type__r7rs__inexact-complex)**:
    * [`inexact-real`](../../r7rs/types/inexact-real.md#type__r7rs__inexact-real):
      * ...
    * **[`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf)**:
      * ...
    * **[`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan)**:
      * ...
  * **[`complex-not-zero`](../../r7rs/types/complex-not-zero.md#type__r7rs__complex-not-zero)**:
    * [`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero):
      * ...
    * **[`complex-not-zero-not-nan`](../../r7rs/types/complex-not-zero-not-nan.md#type__r7rs__complex-not-zero-not-nan)**:
      * ...
  * **[`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf)**:
    * [`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf):
      * ...
    * **[`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan)**:
      * ...
    * [`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf):
      * ...
  * **[`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan)**:
    * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan):
      * ...
    * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan):
      * ...
    * [`complex-not-zero-not-nan`](../../r7rs/types/complex-not-zero-not-nan.md#type__r7rs__complex-not-zero-not-nan):
      * ...
    * [`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan):
      * ...
* **[`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number)**:
  * [`inexact-complex`](../../r7rs/types/inexact-complex.md#type__r7rs__inexact-complex):
    * [`inexact-real`](../../r7rs/types/inexact-real.md#type__r7rs__inexact-real):
      * ...
    * [`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf):
      * ...
    * [`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan):
      * ...
  * **[`inexact-number-not-inf`](../../r7rs/types/inexact-number-not-inf.md#type__r7rs__inexact-number-not-inf)**:
    * **[`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan)**:
      * ...
    * [`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf):
      * ...
    * **[`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan)**:
      * ...
  * **[`inexact-number-not-nan`](../../r7rs/types/inexact-number-not-nan.md#type__r7rs__inexact-number-not-nan)**:
    * **[`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf)**:
      * ...
    * [`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan):
      * ...
    * [`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan):
      * ...
* **[`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero)**:
  * [`complex-not-zero`](../../r7rs/types/complex-not-zero.md#type__r7rs__complex-not-zero):
    * [`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero):
      * ...
    * [`complex-not-zero-not-nan`](../../r7rs/types/complex-not-zero-not-nan.md#type__r7rs__complex-not-zero-not-nan):
      * ...
  * **[`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan)**:
    * [`complex-not-zero-not-nan`](../../r7rs/types/complex-not-zero-not-nan.md#type__r7rs__complex-not-zero-not-nan):
      * ...
    * **[`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive)**:
      * ...
    * **[`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative)**:
      * ...
* **[`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf)**:
  * [`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf):
    * [`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf):
      * ...
    * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan):
      * ...
    * [`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf):
      * ...
  * **[`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan)**:
    * **[`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number)**:
      * ...
    * **[`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero)**:
      * ...
    * **[`number-even`](../../r7rs/types/number-even.md#type__r7rs__number-even)**;
    * **[`number-odd`](../../r7rs/types/number-odd.md#type__r7rs__number-odd)**;
    * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan):
      * ...
    * [`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan):
      * ...
  * **[`number-positive-or-zero-not-inf`](../../r7rs/types/number-positive-or-zero-not-inf.md#type__r7rs__number-positive-or-zero-not-inf)**:
    * [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero):
      * ...
    * **[`number-positive-not-inf`](../../r7rs/types/number-positive-not-inf.md#type__r7rs__number-positive-not-inf)**:
      * ...
    * **[`real-positive-or-zero-not-inf`](../../r7rs/types/real-positive-or-zero-not-inf.md#type__r7rs__real-positive-or-zero-not-inf)**:
      * ...
  * **[`number-negative-or-zero-not-inf`](../../r7rs/types/number-negative-or-zero-not-inf.md#type__r7rs__number-negative-or-zero-not-inf)**:
    * [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero):
      * ...
    * **[`number-negative-not-inf`](../../r7rs/types/number-negative-not-inf.md#type__r7rs__number-negative-not-inf)**:
      * ...
    * **[`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf)**:
      * ...
  * [`inexact-number-not-inf`](../../r7rs/types/inexact-number-not-inf.md#type__r7rs__inexact-number-not-inf):
    * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan):
      * ...
    * [`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf):
      * ...
    * [`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan):
      * ...
* **[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan)**:
  * [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan):
    * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan):
      * ...
    * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan):
      * ...
    * [`complex-not-zero-not-nan`](../../r7rs/types/complex-not-zero-not-nan.md#type__r7rs__complex-not-zero-not-nan):
      * ...
    * [`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan):
      * ...
  * [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan):
    * [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number):
      * ...
    * [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero):
      * ...
    * [`number-even`](../../r7rs/types/number-even.md#type__r7rs__number-even);
    * [`number-odd`](../../r7rs/types/number-odd.md#type__r7rs__number-odd);
    * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan):
      * ...
    * [`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan):
      * ...
  * [`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan):
    * [`complex-not-zero-not-nan`](../../r7rs/types/complex-not-zero-not-nan.md#type__r7rs__complex-not-zero-not-nan):
      * ...
    * [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive):
      * ...
    * [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative):
      * ...
  * **[`number-positive-or-zero`](../../r7rs/types/number-positive-or-zero.md#type__r7rs__number-positive-or-zero)**:
    * [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive):
      * ...
    * [`number-positive-or-zero-not-inf`](../../r7rs/types/number-positive-or-zero-not-inf.md#type__r7rs__number-positive-or-zero-not-inf):
      * ...
    * **[`real-positive-or-zero`](../../r7rs/types/real-positive-or-zero.md#type__r7rs__real-positive-or-zero)**:
      * ...
  * **[`number-negative-or-zero`](../../r7rs/types/number-negative-or-zero.md#type__r7rs__number-negative-or-zero)**:
    * [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative):
      * ...
    * [`number-negative-or-zero-not-inf`](../../r7rs/types/number-negative-or-zero-not-inf.md#type__r7rs__number-negative-or-zero-not-inf):
      * ...
    * **[`real-negative-or-zero`](../../r7rs/types/real-negative-or-zero.md#type__r7rs__real-negative-or-zero)**:
      * ...
  * [`inexact-number-not-nan`](../../r7rs/types/inexact-number-not-nan.md#type__r7rs__inexact-number-not-nan):
    * [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf):
      * ...
    * [`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan):
      * ...
    * [`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan):
      * ...


<a id='type__r7rs__number__super-types'></a>

#### Super-types

 * [(none)](../../r7rs/types/_index.md#toc__r7rs__types);


<a id='type__r7rs__number__sub-types'></a>

#### Sub-types

 * [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
 * [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
 * [`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
 * [`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);


<a id='type__r7rs__number__sub-types-recursive'></a>

##### Sub-types recursive

 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`rational`](../../r7rs/types/rational.md#type__r7rs__rational);
 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * [`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);
 * [`exact-complex`](../../r7rs/types/exact-complex.md#type__r7rs__exact-complex);
 * [`exact-real`](../../r7rs/types/exact-real.md#type__r7rs__exact-real);
 * [`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational);
 * [`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer);
 * [`inexact-complex`](../../r7rs/types/inexact-complex.md#type__r7rs__inexact-complex);
 * [`inexact-real`](../../r7rs/types/inexact-real.md#type__r7rs__inexact-real);
 * [`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);
 * [`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);
 * [`number-zero`](../../r7rs/types/number-zero.md#type__r7rs__number-zero);
 * [`complex-zero`](../../r7rs/types/complex-zero.md#type__r7rs__complex-zero);
 * [`real-zero`](../../r7rs/types/real-zero.md#type__r7rs__real-zero);
 * [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero);
 * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero);
 * [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
 * [`complex-not-zero`](../../r7rs/types/complex-not-zero.md#type__r7rs__complex-not-zero);
 * [`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero);
 * [`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero);
 * [`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero);
 * [`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero);
 * [`number-even`](../../r7rs/types/number-even.md#type__r7rs__number-even);
 * [`number-odd`](../../r7rs/types/number-odd.md#type__r7rs__number-odd);
 * [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
 * [`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf);
 * [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
 * [`complex-nan`](../../r7rs/types/complex-nan.md#type__r7rs__complex-nan);
 * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
 * [`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf);
 * [`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf);
 * [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
 * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);
 * [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
 * [`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan);
 * [`complex-not-zero-not-nan`](../../r7rs/types/complex-not-zero-not-nan.md#type__r7rs__complex-not-zero-not-nan);
 * [`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan);
 * [`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
 * [`number-positive-not-inf`](../../r7rs/types/number-positive-not-inf.md#type__r7rs__number-positive-not-inf);
 * [`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
 * [`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf);
 * [`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive);
 * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);
 * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
 * [`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
 * [`number-negative-not-inf`](../../r7rs/types/number-negative-not-inf.md#type__r7rs__number-negative-not-inf);
 * [`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
 * [`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf);
 * [`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);
 * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
 * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
 * [`number-positive-or-zero`](../../r7rs/types/number-positive-or-zero.md#type__r7rs__number-positive-or-zero);
 * [`number-positive-or-zero-not-inf`](../../r7rs/types/number-positive-or-zero-not-inf.md#type__r7rs__number-positive-or-zero-not-inf);
 * [`real-positive-or-zero`](../../r7rs/types/real-positive-or-zero.md#type__r7rs__real-positive-or-zero);
 * [`real-positive-or-zero-not-inf`](../../r7rs/types/real-positive-or-zero-not-inf.md#type__r7rs__real-positive-or-zero-not-inf);
 * [`rational-positive-or-zero`](../../r7rs/types/rational-positive-or-zero.md#type__r7rs__rational-positive-or-zero);
 * [`integer-positive-or-zero`](../../r7rs/types/integer-positive-or-zero.md#type__r7rs__integer-positive-or-zero);
 * [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
 * [`number-negative-or-zero`](../../r7rs/types/number-negative-or-zero.md#type__r7rs__number-negative-or-zero);
 * [`number-negative-or-zero-not-inf`](../../r7rs/types/number-negative-or-zero-not-inf.md#type__r7rs__number-negative-or-zero-not-inf);
 * [`real-negative-or-zero`](../../r7rs/types/real-negative-or-zero.md#type__r7rs__real-negative-or-zero);
 * [`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf);
 * [`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero);
 * [`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero);
 * [`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero);
 * [`inexact-number-not-inf`](../../r7rs/types/inexact-number-not-inf.md#type__r7rs__inexact-number-not-inf);
 * [`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf);
 * [`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf);
 * [`inexact-number-not-nan`](../../r7rs/types/inexact-number-not-nan.md#type__r7rs__inexact-number-not-nan);
 * [`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan);
 * [`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan);
 * [`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan);
 * [`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan);
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


<a id='type__r7rs__number__referent-definitions-input'></a>

#### Referent definitions as input

 * [`number?`](../../r7rs/definitions/number_3f.md#definition__r7rs__number_3f);
 * [`integer?`](../../r7rs/definitions/integer_3f.md#definition__r7rs__integer_3f);
 * [`real?`](../../r7rs/definitions/real_3f.md#definition__r7rs__real_3f);
 * [`rational?`](../../r7rs/definitions/rational_3f.md#definition__r7rs__rational_3f);
 * [`complex?`](../../r7rs/definitions/complex_3f.md#definition__r7rs__complex_3f);
 * [`exact?`](../../r7rs/definitions/exact_3f.md#definition__r7rs__exact_3f);
 * [`inexact?`](../../r7rs/definitions/inexact_3f.md#definition__r7rs__inexact_3f);
 * [`exact-integer?`](../../r7rs/definitions/exact-integer_3f.md#definition__r7rs__exact-integer_3f);
 * [`zero?`](../../r7rs/definitions/zero_3f.md#definition__r7rs__zero_3f);
 * [`positive?`](../../r7rs/definitions/positive_3f.md#definition__r7rs__positive_3f);
 * [`negative?`](../../r7rs/definitions/negative_3f.md#definition__r7rs__negative_3f);
 * [`odd?`](../../r7rs/definitions/odd_3f.md#definition__r7rs__odd_3f);
 * [`even?`](../../r7rs/definitions/even_3f.md#definition__r7rs__even_3f);
 * [`=`](../../r7rs/definitions/ZZZZ__3d.md#definition__r7rs__ZZZZ__3d);
 * [`<`](../../r7rs/definitions/ZZZZ__3c.md#definition__r7rs__ZZZZ__3c);
 * [`>`](../../r7rs/definitions/ZZZZ__3e.md#definition__r7rs__ZZZZ__3e);
 * [`<=`](../../r7rs/definitions/ZZZZ__3c_3d.md#definition__r7rs__ZZZZ__3c_3d);
 * [`>=`](../../r7rs/definitions/ZZZZ__3e_3d.md#definition__r7rs__ZZZZ__3e_3d);
 * [`+`](../../r7rs/definitions/ZZZZ__2b.md#definition__r7rs__ZZZZ__2b);
 * [`-`](../../r7rs/definitions/ZZZZ__2d.md#definition__r7rs__ZZZZ__2d);
 * [`*`](../../r7rs/definitions/ZZZZ__2a.md#definition__r7rs__ZZZZ__2a);
 * [`/`](../../r7rs/definitions/ZZZZ__2f.md#definition__r7rs__ZZZZ__2f);
 * [`floor/`](../../r7rs/definitions/floor_2f.md#definition__r7rs__floor_2f);
 * [`floor-quotient`](../../r7rs/definitions/floor-quotient.md#definition__r7rs__floor-quotient);
 * [`floor-remainder`](../../r7rs/definitions/floor-remainder.md#definition__r7rs__floor-remainder);
 * [`truncate/`](../../r7rs/definitions/truncate_2f.md#definition__r7rs__truncate_2f);
 * [`truncate-quotient`](../../r7rs/definitions/truncate-quotient.md#definition__r7rs__truncate-quotient);
 * [`truncate-remainder`](../../r7rs/definitions/truncate-remainder.md#definition__r7rs__truncate-remainder);
 * [`min`](../../r7rs/definitions/min.md#definition__r7rs__min);
 * [`max`](../../r7rs/definitions/max.md#definition__r7rs__max);
 * [`gcd`](../../r7rs/definitions/gcd.md#definition__r7rs__gcd);
 * [`lcm`](../../r7rs/definitions/lcm.md#definition__r7rs__lcm);
 * [`expt`](../../r7rs/definitions/expt.md#definition__r7rs__expt);
 * [`square`](../../r7rs/definitions/square.md#definition__r7rs__square);
 * [`inexact`](../../r7rs/definitions/inexact.md#definition__r7rs__inexact);
 * [`number->string`](../../r7rs/definitions/number-_3e_string.md#definition__r7rs__number-_3e_string);


<a id='type__r7rs__number__referent-definitions-output'></a>

#### Referent definitions as output

 * [`+`](../../r7rs/definitions/ZZZZ__2b.md#definition__r7rs__ZZZZ__2b);
 * [`-`](../../r7rs/definitions/ZZZZ__2d.md#definition__r7rs__ZZZZ__2d);
 * [`*`](../../r7rs/definitions/ZZZZ__2a.md#definition__r7rs__ZZZZ__2a);
 * [`/`](../../r7rs/definitions/ZZZZ__2f.md#definition__r7rs__ZZZZ__2f);


<a id='type__r7rs__number__referent-definitions-output-recursive'></a>

#### Referent definitions as output (recursive)

 * [`rationalize`](../../r7rs/definitions/rationalize.md#definition__r7rs__rationalize);
 * [`floor`](../../r7rs/definitions/floor.md#definition__r7rs__floor);
 * [`ceiling`](../../r7rs/definitions/ceiling.md#definition__r7rs__ceiling);
 * [`truncate`](../../r7rs/definitions/truncate.md#definition__r7rs__truncate);
 * [`round`](../../r7rs/definitions/round.md#definition__r7rs__round);
 * [`numerator`](../../r7rs/definitions/numerator.md#definition__r7rs__numerator);
 * [`denominator`](../../r7rs/definitions/denominator.md#definition__r7rs__denominator);
 * [`exact`](../../r7rs/definitions/exact.md#definition__r7rs__exact);
 * [`inexact`](../../r7rs/definitions/inexact.md#definition__r7rs__inexact);
 * [`abs`](../../r7rs/definitions/abs.md#definition__r7rs__abs);
 * [`exact-integer-sqrt`](../../r7rs/definitions/exact-integer-sqrt.md#definition__r7rs__exact-integer-sqrt);
 * [`sqrt`](../../r7rs/definitions/sqrt.md#definition__r7rs__sqrt);
 * [`floor/`](../../r7rs/definitions/floor_2f.md#definition__r7rs__floor_2f);
 * [`floor-quotient`](../../r7rs/definitions/floor-quotient.md#definition__r7rs__floor-quotient);
 * [`floor-remainder`](../../r7rs/definitions/floor-remainder.md#definition__r7rs__floor-remainder);
 * [`truncate/`](../../r7rs/definitions/truncate_2f.md#definition__r7rs__truncate_2f);
 * [`truncate-quotient`](../../r7rs/definitions/truncate-quotient.md#definition__r7rs__truncate-quotient);
 * [`truncate-remainder`](../../r7rs/definitions/truncate-remainder.md#definition__r7rs__truncate-remainder);
 * [`min`](../../r7rs/definitions/min.md#definition__r7rs__min);
 * [`max`](../../r7rs/definitions/max.md#definition__r7rs__max);
 * [`gcd`](../../r7rs/definitions/gcd.md#definition__r7rs__gcd);
 * [`lcm`](../../r7rs/definitions/lcm.md#definition__r7rs__lcm);
 * [`expt`](../../r7rs/definitions/expt.md#definition__r7rs__expt);
 * [`square`](../../r7rs/definitions/square.md#definition__r7rs__square);
 * [`exp`](../../r7rs/definitions/exp.md#definition__r7rs__exp);
 * [`log`](../../r7rs/definitions/log.md#definition__r7rs__log);
 * [`sin`](../../r7rs/definitions/sin.md#definition__r7rs__sin);
 * [`cos`](../../r7rs/definitions/cos.md#definition__r7rs__cos);
 * [`tan`](../../r7rs/definitions/tan.md#definition__r7rs__tan);
 * [`asin`](../../r7rs/definitions/asin.md#definition__r7rs__asin);
 * [`acos`](../../r7rs/definitions/acos.md#definition__r7rs__acos);
 * [`atan`](../../r7rs/definitions/atan.md#definition__r7rs__atan);
 * [`make-rectangular`](../../r7rs/definitions/make-rectangular.md#definition__r7rs__make-rectangular);
 * [`make-polar`](../../r7rs/definitions/make-polar.md#definition__r7rs__make-polar);
 * [`real-part`](../../r7rs/definitions/real-part.md#definition__r7rs__real-part);
 * [`imag-part`](../../r7rs/definitions/imag-part.md#definition__r7rs__imag-part);
 * [`angle`](../../r7rs/definitions/angle.md#definition__r7rs__angle);
 * [`magnitude`](../../r7rs/definitions/magnitude.md#definition__r7rs__magnitude);
 * [`digit-value`](../../r7rs/definitions/digit-value.md#definition__r7rs__digit-value);
 * [`char->integer`](../../r7rs/definitions/char-_3e_integer.md#definition__r7rs__char-_3e_integer);
 * [`length`](../../r7rs/definitions/length.md#definition__r7rs__length);
 * [`vector-length`](../../r7rs/definitions/vector-length.md#definition__r7rs__vector-length);
 * [`string-length`](../../r7rs/definitions/string-length.md#definition__r7rs__string-length);
 * [`bytevector-length`](../../r7rs/definitions/bytevector-length.md#definition__r7rs__bytevector-length);
 * [`bytevector-u8-ref`](../../r7rs/definitions/bytevector-u8-ref.md#definition__r7rs__bytevector-u8-ref);

Note:  These definitions produce an output that is a sub-type.


<a id='type__r7rs__number__predicate'></a>

#### Predicate

````
number?
````


<a id='type__r7rs__number__description'></a>

#### Description

> It is important to distinguish between mathematical numbers, the
> Scheme numbers that attempt to model them, the machine representations
> used to implement the Scheme numbers, and notations used to write numbers.
> This report uses the types `number`, `complex`, `real`,
> `rational`, and `integer` to refer to both mathematical numbers
> and Scheme numbers.
> 
> 
> ##### Numerical types
> 
> Mathematically, numbers are arranged into a tower of subtypes
> in which each level is a subset of the level above it:
>   * number
>   * complex number
>   * real number
>   * rational number
>   * integer
> 
> For example, 3 is an integer.  Therefore 3 is also a rational,
> a real, and a complex number.  The same is true of the Scheme numbers
> that model 3.  For Scheme numbers, these types are defined by the
> predicates `number?`, `complex?`, `real?`, `rational?`,
> and `integer?`.
> 
> There is no simple relationship between a number's type and its
> representation inside a computer.  Although most implementations of
> Scheme will offer at least two different representations of 3, these
> different representations denote the same integer.
> 
> Scheme's numerical operations treat numbers as abstract data, as
> independent of their representation as possible.  Although an implementation
> of Scheme may use multiple internal representations of
> numbers, this ought not to be apparent to a casual programmer writing
> simple programs.
> 
> 
> ##### Exactness
> 
> It is useful to distinguish between numbers that are
> represented exactly and those that might not be.  For example, indexes
> into data structures must be known exactly, as must some polynomial
> coefficients in a symbolic algebra system.  On the other hand, the
> results of measurements are inherently inexact, and irrational numbers
> may be approximated by rational and therefore inexact approximations.
> In order to catch uses of inexact numbers where exact numbers are
> required, Scheme explicitly distinguishes exact from inexact numbers.
> This distinction is orthogonal to the dimension of type.
> 
> A Scheme number is
> `exact` if it was written as an exact constant or was derived from
> __exact__ numbers using only __exact__ operations.  A number is
> `inexact` if it was written as an inexact constant,
> if it was
> derived using __inexact__ ingredients, or if it was derived using
> __inexact__ operations. Thus __inexact__-ness is a contagious
> property of a number.
> 
> In particular, an __exact complex number__ has an exact real part
> and an exact imaginary part; all other complex numbers are
> __inexact complex numbers__.
> 
> If two implementations produce __exact__ results for a
> computation that did not involve __inexact__ intermediate results,
> the two ultimate results will be mathematically equal.  This is
> generally not true of computations involving __inexact__ numbers
> since approximate methods such as floating-point arithmetic may be used,
> but it is the duty of each implementation to make the result as close as
> practical to the mathematically ideal result.
> 
> Rational operations such as `+` should always produce
> __exact__ results when given __exact__ arguments.
> If the operation is unable to produce an __exact__ result,
> then it may either report the violation of an implementation restriction
> or it may silently coerce its
> result to an __inexact__ value.
> However, `(/ 3 4)` must not return the mathematically incorrect value `0`.
> 
> Except for `exact`, the operations described in
> this section must generally return inexact results when given any inexact
> arguments.  An operation may, however, return an __exact__ result if it can
> prove that the value of the result is unaffected by the inexactness of its
> arguments.  For example, multiplication of any number by an __exact__ zero
> may produce an __exact__ zero result, even if the other argument is
> __inexact__.
> 
> Specifically, the expression `(* 0 +inf.0)` may return `0`,
> or `+nan.0`, or report that inexact numbers are not supported,
> or report that non-rational real numbers are not supported, or fail
> silently or noisily in other implementation-specific ways.
> 
> 
> ##### Implementation restrictions
> 
> Implementations of Scheme are not required to implement the whole
> tower of subtypes given in the section "Numerical types",
> but they must implement a coherent subset consistent with both the
> purposes of the implementation and the spirit of the Scheme language.
> For example, implementations in which all numbers are __real__,
> or in which non-__real__ numbers are always __inexact__,
> or in which __exact__ numbers are always __integer__,
> are still quite useful.
> 
> Implementations may also support only a limited range of numbers of
> any type, subject to the requirements of this section.  The supported
> range for __exact__ numbers of any type may be different from the
> supported range for __inexact__ numbers of that type.  For example,
> an implementation that uses IEEE binary double-precision floating-point numbers to represent all its
> __inexact__ __real__ numbers may also
> support a practically unbounded range of __exact__ __integer__'s
> and __rational__'s
> while limiting the range of __inexact__ __real__'s (and therefore
> the range of __inexact__ __integer__'s and __rational__'s)
> to the dynamic range of the IEEE binary double format.
> Furthermore,
> the gaps between the representable __inexact__ __integer__'s and
> __rational__'s are
> likely to be very large in such an implementation as the limits of this
> range are approached.
> 
> An implementation of Scheme must support exact integers
> throughout the range of numbers permitted as indexes of
> lists, vectors, bytevectors, and strings or that result from computing the length of
> one of these.  The `length`, `vector-length`,
> `bytevector-length`, and `string-length` procedures must return an exact
> integer, and it is an error to use anything but an exact integer as an
> index.  Furthermore, any integer constant within the index range, if
> expressed by an exact integer syntax, must be read as an exact
> integer, regardless of any implementation restrictions that apply
> outside this range.  Finally, the procedures listed below will always
> return exact integer results provided all their arguments are exact integers
> and the mathematically expected results are representable as exact integers
> within the implementation:
> 
> ````
> -                     *
> +                     abs
> ceiling               denominator
> exact-integer-sqrt    expt
> floor                 floor/
> floor-quotient        floor-remainder
> gcd                   lcm
> max                   min
> modulo                numerator
> quotient              rationalize
> remainder             round
> square                truncate
> truncate/             truncate-quotient
> truncate-remainder
> ````
> 
> It is recommended, but not required, that implementations support
> __exact__ __integer__'s and __exact__ __rational__'s of
> practically unlimited size and precision, and to implement the
> above procedures and the `/` procedure in
> such a way that they always return __exact__ results when given __exact__
> arguments.  If one of these procedures is unable to deliver an __exact__
> result when given __exact__ arguments, then it may either report a
> violation of an
> implementation restriction or it may silently coerce its result to an
> __inexact__ number; such a coercion can cause an error later.
> Nevertheless, implementations that do not provide __exact__ rational
> numbers should return __inexact__ rational numbers rather than
> reporting an implementation restriction.
> 
> An implementation may use floating-point and other approximate
> representation strategies for __inexact__ numbers.
> This report recommends, but does not require, that
> implementations that use
> floating-point representations
> follow the IEEE 754 standard,
> and that implementations using
> other representations should match or exceed the precision achievable
> using these floating-point standards.
> In particular, the description of transcendental functions in IEEE 754-2008
> should be followed by such implementations, particularly with respect
> to infinities and `NaN`s.
> 
> Although Scheme allows a variety of written
> notations for
> numbers, any particular implementation may support only some of them.
> For example, an implementation in which all numbers are __real__
> need not support the rectangular and polar notations for complex
> numbers.  If an implementation encounters an __exact__ numerical constant that
> it cannot represent as an __exact__ number, then it may either report a
> violation of an implementation restriction or it may silently represent the
> constant by an __inexact__ number.
> 
> 
> ##### Implementation extensions
> 
> Implementations may provide more than one representation of
> floating-point numbers with differing precisions.  In an implementation
> which does so, an inexact result must be represented with at least
> as much precision as is used to express any of the inexact arguments
> to that operation.  Although it is desirable for potentially inexact
> operations such as `sqrt` to produce __exact__ answers when
> applied to __exact__ arguments, if an __exact__ number is operated
> upon so as to produce an __inexact__ result, then the most precise
> representation available must be used.  For example, the value of
> `(sqrt 4)` should be `2`, but in an implementation that provides both
> single and double precision floating point numbers it may be the latter
> but must not be the former.
> 
> It is the programmer's responsibility to avoid using inexact
> number objects with magnitude or significand too large to be
> represented in the implementation.
> 
> In addition, implementations may
> distinguish special numbers called __positive infinity__,
> __negative infinity__, __NaN__, and __negative zero__.
> 
> Positive infinity is regarded as an inexact real (but not rational)
> number that represents an indeterminate value greater than the
> numbers represented by all rational numbers. Negative infinity
> is regarded as an inexact real (but not rational) number that
> represents an indeterminate value less than the numbers represented
> by all rational numbers.
> 
> Adding or multiplying an infinite value by any finite real value results
> in an appropriately signed infinity; however, the sum of positive and
> negative infinities is a `NaN`.  Positive infinity is the reciprocal
> of zero, and negative infinity is the reciprocal of negative zero.
> The behavior of the transcendental functions is sensitive to infinity
> in accordance with IEEE 754.
> 
> A `NaN` is regarded as an inexact real (but not rational) number
> so indeterminate that it might represent any real value, including
> positive or negative infinity, and might even be greater than positive
> infinity or less than negative infinity.
> An implementation that does not support non-real numbers may use `NaN`
> to represent non-real values like `(sqrt -1.0)` and `(asin 2.0)`.
> 
> A `NaN` always compares false to any number, including a `NaN`.
> An arithmetic operation where one operand is `NaN` returns `NaN`, unless the
> implementation can prove that the result would be the same if the `NaN`
> were replaced by any rational number.  Dividing zero by zero results in
> `NaN` unless both zeros are exact.
> 
> IEEE 754 specifies multiple `NaN` values.  Scheme generally does
> not care if there is a single value (bit pattern) for `NaN`,
> or if there are multiple values: if there are multiple `NaN`
> values, or just one, they are all equivalent in terms of Scheme
> computation.
> 
> Negative zero is an inexact real value written `-0.0` and is distinct
> (in the sense of `eqv?`) from `0.0`.  A Scheme implementation
> is not required to distinguish negative zero.  If it does, however, the
> behavior of the transcendental functions is sensitive to the distinction
> in accordance with IEEE 754.
> Specifically, in a Scheme implementing both complex numbers and negative zero,
> the branch cut of the complex logarithm function is such that
> `(imag-part (log -1.0-0.0i))` is `-pi` rather than `pi`.
> 
> Furthermore, the negation of negative zero is ordinary zero and vice
> versa.  This implies that the sum of two or more negative zeros is negative,
> and the result of subtracting (positive) zero from a negative zero is
> likewise negative.  However, numerical comparisons treat negative zero
> as equal to zero.
> 
> Note that both the real and the imaginary parts of a complex number
> can be infinities, `NaN`s, or negative zero.
> 
> 
> ##### Syntax of numerical constants
> 
> The syntax of the written representations for numbers is described formally in the
> section on formal syntax.  Note that case is not significant in numerical
> constants.
> 
> A number can be written in binary, octal, decimal, or
> hexa-decimal by the use of a radix prefix.  The radix prefixes are
> `#b` (binary), `#o` (octal),
> `#d` (decimal), and `#x` (hexa-decimal).  With
> no radix prefix, a number is assumed to be expressed in decimal.
> 
> A
> numerical constant can be specified to be either __exact__ or
> __inexact__ by a prefix.  The prefixes are `#e`
> for __exact__, and `#i` for __inexact__.  An exactness
> prefix can appear before or after any radix prefix that is used.  If
> the written representation of a number has no exactness prefix, the
> constant is
> __inexact__ if it contains a decimal point or an
> exponent.
> Otherwise, it is __exact__.
> 
> In systems with __inexact__ numbers
> of varying precisions it can be useful to specify
> the precision of a constant.  For this purpose,
> implementations may accept numerical constants
> written with an exponent marker that indicates the
> desired precision of the __inexact__
> representation.  If so, the letter `s`, `f`,
> `d`, or `l`, meaning __short__, __single__,
> __double__, or __long__ precision, respectively,
> can be used in place of `e`.
> The default precision has at least as much precision
> as __double__, but
> implementations may allow this default to be set by the user.
> 
> ````
> 3.14159265358979F0
>        Round to single  ---  3.141593
> 0.6L0
>        Extend to long   ---  .600000000000000
> ````
> 
> The numbers positive infinity, negative infinity, and `NaN` are written
> `+inf.0`, `-inf.0` and `+nan.0` respectively.
> `NaN` may also be written `-nan.0`.
> The use of signs in the written representation does not necessarily
> reflect the underlying sign of the `NaN` value, if any.
> Implementations are not required to support these numbers, but if they do,
> they must do so in general conformance with IEEE 754.  However, implementations
> are not required to support signaling `NaN`s, nor to provide a way to distinguish
> between different `NaN`s.
> 
> There are two notations provided for non-real complex numbers:
> the __rectangular notation__
> `a + b i`,
> where `a` is the real part and `b` is the imaginary part;
> and the __polar notation__
> `r @ theta`,
> where `r` is the magnitude and `theta` is the phase (angle) in radians.
> These are related by the equation
> `a + b i = r cos(theta) + (r sin (theta)) i`.
> All of `a`, `b`, `r`, and `theta` are real numbers.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='type__r7rs__number__categories'></a>

#### Categories

 * [`r7rs:types-disjoint`](../../r7rs/categories/r7rs_3a_types-disjoint.md#category__r7rs__r7rs_3a_types-disjoint);
 * [`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);


<a id='type__r7rs__number__categories-recursive'></a>

#### Categories recursive

 * [`r7rs:types`](../../r7rs/categories/r7rs_3a_types.md#category__r7rs__r7rs_3a_types);
 * [`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

