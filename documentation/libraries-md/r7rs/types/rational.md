

<a id='type__r7rs__rational'></a>

# `rational` -- `r7rs` Type


<a id='type__r7rs__rational__sub-types-tree'></a>

#### Sub-types tree

* **[`integer`](../../r7rs/types/integer.md#type__r7rs__integer)**:
  * **[`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer)**:
    * **[`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero)**:
      * ...
    * **[`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero)**:
      * ...
    * **[`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero)**:
      * ...
  * **[`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer)**;
  * **[`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero)**:
    * [`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero):
      * ...
    * **[`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive)**:
      * ...
    * **[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative)**:
      * ...
  * **[`integer-even`](../../r7rs/types/integer-even.md#type__r7rs__integer-even)**;
  * **[`integer-odd`](../../r7rs/types/integer-odd.md#type__r7rs__integer-odd)**;
  * **[`integer-positive-or-zero`](../../r7rs/types/integer-positive-or-zero.md#type__r7rs__integer-positive-or-zero)**:
    * **[`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero)**:
      * ...
    * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive):
      * ...
    * [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero):
      * ...
  * **[`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero)**:
    * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero):
      * ...
    * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative):
      * ...
    * [`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero):
      * ...
* **[`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational)**:
  * [`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer):
    * [`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero):
      * ...
    * [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero):
      * ...
    * [`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero):
      * ...
* **[`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational)**:
  * [`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);
* **[`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero)**:
  * [`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero):
    * [`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero):
      * ...
    * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive):
      * ...
    * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative):
      * ...
  * **[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive)**:
    * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive):
      * ...
  * **[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative)**:
    * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative):
      * ...
* **[`rational-positive-or-zero`](../../r7rs/types/rational-positive-or-zero.md#type__r7rs__rational-positive-or-zero)**:
  * **[`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero)**:
    * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero):
      * ...
  * [`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive):
    * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive):
      * ...
  * [`integer-positive-or-zero`](../../r7rs/types/integer-positive-or-zero.md#type__r7rs__integer-positive-or-zero):
    * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero):
      * ...
    * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive):
      * ...
    * [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero):
      * ...
* **[`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero)**:
  * [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero):
    * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero):
      * ...
  * [`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative):
    * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative):
      * ...
  * [`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero):
    * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero):
      * ...
    * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative):
      * ...
    * [`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero):
      * ...


<a id='type__r7rs__rational__super-types'></a>

#### Super-types

 * [`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);


<a id='type__r7rs__rational__super-types-recursive'></a>

##### Super-types recursive

 * [`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);
 * [`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
 * [`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
 * [`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf);
 * [`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
 * [`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
 * [`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);


<a id='type__r7rs__rational__sub-types'></a>

#### Sub-types

 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
 * [`exact-rational`](../../r7rs/types/exact-rational.md#type__r7rs__exact-rational);
 * [`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);
 * [`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero);
 * [`rational-positive-or-zero`](../../r7rs/types/rational-positive-or-zero.md#type__r7rs__rational-positive-or-zero);
 * [`rational-negative-or-zero`](../../r7rs/types/rational-negative-or-zero.md#type__r7rs__rational-negative-or-zero);


<a id='type__r7rs__rational__sub-types-recursive'></a>

##### Sub-types recursive

 * [`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer);
 * [`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);
 * [`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero);
 * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero);
 * [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
 * [`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero);
 * [`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero);
 * [`integer-even`](../../r7rs/types/integer-even.md#type__r7rs__integer-even);
 * [`integer-odd`](../../r7rs/types/integer-odd.md#type__r7rs__integer-odd);
 * [`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive);
 * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);
 * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
 * [`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);
 * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
 * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
 * [`integer-positive-or-zero`](../../r7rs/types/integer-positive-or-zero.md#type__r7rs__integer-positive-or-zero);
 * [`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
 * [`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero);
 * [`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero);
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


<a id='type__r7rs__rational__referent-definitions-input'></a>

#### Referent definitions as input

 * [`number?`](../../r7rs/definitions/number_3f.md#definition__r7rs__number_3f);
 * [`integer?`](../../r7rs/definitions/integer_3f.md#definition__r7rs__integer_3f);
 * [`real?`](../../r7rs/definitions/real_3f.md#definition__r7rs__real_3f);
 * [`rational?`](../../r7rs/definitions/rational_3f.md#definition__r7rs__rational_3f);
 * [`complex?`](../../r7rs/definitions/complex_3f.md#definition__r7rs__complex_3f);


<a id='type__r7rs__rational__referent-definitions-input-recursive'></a>

#### Referent definitions as input (recursive)

 * [`exact?`](../../r7rs/definitions/exact_3f.md#definition__r7rs__exact_3f);
 * [`inexact?`](../../r7rs/definitions/inexact_3f.md#definition__r7rs__inexact_3f);
 * [`exact-integer?`](../../r7rs/definitions/exact-integer_3f.md#definition__r7rs__exact-integer_3f);
 * [`zero?`](../../r7rs/definitions/zero_3f.md#definition__r7rs__zero_3f);
 * [`=`](../../r7rs/definitions/ZZZZ__3d.md#definition__r7rs__ZZZZ__3d);
 * [`+`](../../r7rs/definitions/ZZZZ__2b.md#definition__r7rs__ZZZZ__2b);
 * [`-`](../../r7rs/definitions/ZZZZ__2d.md#definition__r7rs__ZZZZ__2d);
 * [`*`](../../r7rs/definitions/ZZZZ__2a.md#definition__r7rs__ZZZZ__2a);
 * [`/`](../../r7rs/definitions/ZZZZ__2f.md#definition__r7rs__ZZZZ__2f);
 * [`expt`](../../r7rs/definitions/expt.md#definition__r7rs__expt);
 * [`square`](../../r7rs/definitions/square.md#definition__r7rs__square);
 * [`inexact`](../../r7rs/definitions/inexact.md#definition__r7rs__inexact);
 * [`sqrt`](../../r7rs/definitions/sqrt.md#definition__r7rs__sqrt);
 * [`exp`](../../r7rs/definitions/exp.md#definition__r7rs__exp);
 * [`log`](../../r7rs/definitions/log.md#definition__r7rs__log);
 * [`sin`](../../r7rs/definitions/sin.md#definition__r7rs__sin);
 * [`cos`](../../r7rs/definitions/cos.md#definition__r7rs__cos);
 * [`tan`](../../r7rs/definitions/tan.md#definition__r7rs__tan);
 * [`asin`](../../r7rs/definitions/asin.md#definition__r7rs__asin);
 * [`acos`](../../r7rs/definitions/acos.md#definition__r7rs__acos);
 * [`atan`](../../r7rs/definitions/atan.md#definition__r7rs__atan);
 * [`number->string`](../../r7rs/definitions/number-_3e_string.md#definition__r7rs__number-_3e_string);
 * [`<`](../../vonuvoli/definitions/ZZZZ__3c.md#definition__vonuvoli__ZZZZ__3c);
 * [`<=`](../../vonuvoli/definitions/ZZZZ__3c_3d.md#definition__vonuvoli__ZZZZ__3c_3d);
 * [`>=`](../../vonuvoli/definitions/ZZZZ__3e_3d.md#definition__vonuvoli__ZZZZ__3e_3d);
 * [`>`](../../vonuvoli/definitions/ZZZZ__3e.md#definition__vonuvoli__ZZZZ__3e);
 * [`positive?`](../../r7rs/definitions/positive_3f.md#definition__r7rs__positive_3f);
 * [`negative?`](../../r7rs/definitions/negative_3f.md#definition__r7rs__negative_3f);
 * [`abs`](../../r7rs/definitions/abs.md#definition__r7rs__abs);
 * [`floor/`](../../r7rs/definitions/floor_2f.md#definition__r7rs__floor_2f);
 * [`floor-quotient`](../../r7rs/definitions/floor-quotient.md#definition__r7rs__floor-quotient);
 * [`floor-remainder`](../../r7rs/definitions/floor-remainder.md#definition__r7rs__floor-remainder);
 * [`truncate/`](../../r7rs/definitions/truncate_2f.md#definition__r7rs__truncate_2f);
 * [`truncate-quotient`](../../r7rs/definitions/truncate-quotient.md#definition__r7rs__truncate-quotient);
 * [`truncate-remainder`](../../r7rs/definitions/truncate-remainder.md#definition__r7rs__truncate-remainder);
 * [`min`](../../r7rs/definitions/min.md#definition__r7rs__min);
 * [`max`](../../r7rs/definitions/max.md#definition__r7rs__max);
 * [`exact`](../../r7rs/definitions/exact.md#definition__r7rs__exact);
 * [`finite?`](../../r7rs/definitions/finite_3f.md#definition__r7rs__finite_3f);
 * [`infinite?`](../../r7rs/definitions/infinite_3f.md#definition__r7rs__infinite_3f);
 * [`nan?`](../../r7rs/definitions/nan_3f.md#definition__r7rs__nan_3f);
 * [`real-part`](../../r7rs/definitions/real-part.md#definition__r7rs__real-part);
 * [`imag-part`](../../r7rs/definitions/imag-part.md#definition__r7rs__imag-part);
 * [`magnitude`](../../r7rs/definitions/magnitude.md#definition__r7rs__magnitude);
 * [`angle`](../../r7rs/definitions/angle.md#definition__r7rs__angle);
 * [`floor`](../../r7rs/definitions/floor.md#definition__r7rs__floor);
 * [`ceiling`](../../r7rs/definitions/ceiling.md#definition__r7rs__ceiling);
 * [`truncate`](../../r7rs/definitions/truncate.md#definition__r7rs__truncate);
 * [`round`](../../r7rs/definitions/round.md#definition__r7rs__round);
 * [`rationalize`](../../r7rs/definitions/rationalize.md#definition__r7rs__rationalize);
 * [`make-rectangular`](../../r7rs/definitions/make-rectangular.md#definition__r7rs__make-rectangular);
 * [`make-polar`](../../r7rs/definitions/make-polar.md#definition__r7rs__make-polar);

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__rational__referent-definitions-output'></a>

#### Referent definitions as output

 * [`rationalize`](../../r7rs/definitions/rationalize.md#definition__r7rs__rationalize);


<a id='type__r7rs__rational__referent-definitions-output-recursive'></a>

#### Referent definitions as output (recursive)

 * [`floor`](../../r7rs/definitions/floor.md#definition__r7rs__floor);
 * [`ceiling`](../../r7rs/definitions/ceiling.md#definition__r7rs__ceiling);
 * [`truncate`](../../r7rs/definitions/truncate.md#definition__r7rs__truncate);
 * [`round`](../../r7rs/definitions/round.md#definition__r7rs__round);
 * [`gcd`](../../r7rs/definitions/gcd.md#definition__r7rs__gcd);
 * [`lcm`](../../r7rs/definitions/lcm.md#definition__r7rs__lcm);
 * [`numerator`](../../r7rs/definitions/numerator.md#definition__r7rs__numerator);
 * [`exact-integer-sqrt`](../../r7rs/definitions/exact-integer-sqrt.md#definition__r7rs__exact-integer-sqrt);
 * [`denominator`](../../r7rs/definitions/denominator.md#definition__r7rs__denominator);
 * [`digit-value`](../../r7rs/definitions/digit-value.md#definition__r7rs__digit-value);
 * [`char->integer`](../../r7rs/definitions/char-_3e_integer.md#definition__r7rs__char-_3e_integer);
 * [`length`](../../r7rs/definitions/length.md#definition__r7rs__length);
 * [`vector-length`](../../r7rs/definitions/vector-length.md#definition__r7rs__vector-length);
 * [`string-length`](../../r7rs/definitions/string-length.md#definition__r7rs__string-length);
 * [`bytevector-length`](../../r7rs/definitions/bytevector-length.md#definition__r7rs__bytevector-length);
 * [`bytevector-u8-ref`](../../r7rs/definitions/bytevector-u8-ref.md#definition__r7rs__bytevector-u8-ref);
 * [`bytevector-u8-set!`](../../vonuvoli/definitions/bytevector-u8-set_21.md#definition__vonuvoli__bytevector-u8-set_21);

Note:  These definitions produce an output that is a sub-type.


<a id='type__r7rs__rational__predicate'></a>

#### Predicate

````
rational?
````


<a id='type__r7rs__rational__categories'></a>

#### Categories

 * [`types-numbers`](../../r7rs/categories/types-numbers.md#category__r7rs__types-numbers);


<a id='type__r7rs__rational__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

