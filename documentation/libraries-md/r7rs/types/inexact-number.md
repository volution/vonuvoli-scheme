

<a id='type__r7rs__inexact-number'></a>

# `inexact-number` -- `r7rs` Type


<a id='type__r7rs__inexact-number__sub-types-tree'></a>

#### Sub-types tree

* **[`inexact-complex`](../../r7rs/types/inexact-complex.md#type__r7rs__inexact-complex)**:
  * **[`inexact-real`](../../r7rs/types/inexact-real.md#type__r7rs__inexact-real)**:
    * **[`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf)**:
      * ...
    * **[`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan)**:
      * ...
  * **[`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf)**:
    * **[`complex-nan`](../../r7rs/types/complex-nan.md#type__r7rs__complex-nan)**:
      * ...
    * [`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf):
      * ...
    * **[`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan)**:
      * ...
  * **[`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan)**:
    * **[`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf)**:
      * ...
    * [`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan):
      * ...
    * [`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan):
      * ...
* **[`inexact-number-not-inf`](../../r7rs/types/inexact-number-not-inf.md#type__r7rs__inexact-number-not-inf)**:
  * **[`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan)**:
    * [`complex-nan`](../../r7rs/types/complex-nan.md#type__r7rs__complex-nan):
      * ...
  * [`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf):
    * [`complex-nan`](../../r7rs/types/complex-nan.md#type__r7rs__complex-nan):
      * ...
    * [`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf):
      * ...
    * [`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan):
      * ...
  * **[`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan)**:
    * [`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan):
      * ...
* **[`inexact-number-not-nan`](../../r7rs/types/inexact-number-not-nan.md#type__r7rs__inexact-number-not-nan)**:
  * **[`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf)**:
    * [`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf):
      * ...
  * [`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan):
    * [`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf):
      * ...
    * [`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan):
      * ...
    * [`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan):
      * ...
  * [`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan):
    * [`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan):
      * ...


<a id='type__r7rs__inexact-number__super-types'></a>

#### Super-types

 * [`number`](../../r7rs/types/number.md#type__r7rs__number);


<a id='type__r7rs__inexact-number__sub-types'></a>

#### Sub-types

 * [`inexact-complex`](../../r7rs/types/inexact-complex.md#type__r7rs__inexact-complex);
 * [`inexact-number-not-inf`](../../r7rs/types/inexact-number-not-inf.md#type__r7rs__inexact-number-not-inf);
 * [`inexact-number-not-nan`](../../r7rs/types/inexact-number-not-nan.md#type__r7rs__inexact-number-not-nan);


<a id='type__r7rs__inexact-number__sub-types-recursive'></a>

##### Sub-types recursive

 * [`inexact-real`](../../r7rs/types/inexact-real.md#type__r7rs__inexact-real);
 * [`inexact-rational`](../../r7rs/types/inexact-rational.md#type__r7rs__inexact-rational);
 * [`inexact-integer`](../../r7rs/types/inexact-integer.md#type__r7rs__inexact-integer);
 * [`number-inf`](../../r7rs/types/number-inf.md#type__r7rs__number-inf);
 * [`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf);
 * [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);
 * [`number-nan`](../../r7rs/types/number-nan.md#type__r7rs__number-nan);
 * [`complex-nan`](../../r7rs/types/complex-nan.md#type__r7rs__complex-nan);
 * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
 * [`inexact-complex-not-inf`](../../r7rs/types/inexact-complex-not-inf.md#type__r7rs__inexact-complex-not-inf);
 * [`inexact-real-not-inf`](../../r7rs/types/inexact-real-not-inf.md#type__r7rs__inexact-real-not-inf);
 * [`inexact-complex-not-nan`](../../r7rs/types/inexact-complex-not-nan.md#type__r7rs__inexact-complex-not-nan);
 * [`inexact-real-not-nan`](../../r7rs/types/inexact-real-not-nan.md#type__r7rs__inexact-real-not-nan);
 * [`inexact-number-not-inf-not-nan`](../../r7rs/types/inexact-number-not-inf-not-nan.md#type__r7rs__inexact-number-not-inf-not-nan);
 * [`inexact-complex-not-inf-not-nan`](../../r7rs/types/inexact-complex-not-inf-not-nan.md#type__r7rs__inexact-complex-not-inf-not-nan);
 * [`inexact-real-not-inf-not-nan`](../../r7rs/types/inexact-real-not-inf-not-nan.md#type__r7rs__inexact-real-not-inf-not-nan);


<a id='type__r7rs__inexact-number__referent-definitions-input'></a>

#### Referent definitions as input

 * [`exact?`](../../r7rs/definitions/exact_3f.md#definition__r7rs__exact_3f);
 * [`inexact?`](../../r7rs/definitions/inexact_3f.md#definition__r7rs__inexact_3f);
 * [`exact-integer?`](../../r7rs/definitions/exact-integer_3f.md#definition__r7rs__exact-integer_3f);


<a id='type__r7rs__inexact-number__referent-definitions-input-recursive'></a>

#### Referent definitions as input (recursive)

 * [`number?`](../../r7rs/definitions/number_3f.md#definition__r7rs__number_3f);
 * [`integer?`](../../r7rs/definitions/integer_3f.md#definition__r7rs__integer_3f);
 * [`real?`](../../r7rs/definitions/real_3f.md#definition__r7rs__real_3f);
 * [`rational?`](../../r7rs/definitions/rational_3f.md#definition__r7rs__rational_3f);
 * [`complex?`](../../r7rs/definitions/complex_3f.md#definition__r7rs__complex_3f);
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

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__inexact-number__referent-definitions-output'></a>

#### Referent definitions as output

 * [`inexact`](../../r7rs/definitions/inexact.md#definition__r7rs__inexact);


<a id='type__r7rs__inexact-number__referent-definitions-output-recursive'></a>

#### Referent definitions as output (recursive)

 * [`floor`](../../r7rs/definitions/floor.md#definition__r7rs__floor);
 * [`ceiling`](../../r7rs/definitions/ceiling.md#definition__r7rs__ceiling);
 * [`truncate`](../../r7rs/definitions/truncate.md#definition__r7rs__truncate);
 * [`round`](../../r7rs/definitions/round.md#definition__r7rs__round);
 * [`+`](../../r7rs/definitions/ZZZZ__2b.md#definition__r7rs__ZZZZ__2b);
 * [`-`](../../r7rs/definitions/ZZZZ__2d.md#definition__r7rs__ZZZZ__2d);
 * [`*`](../../r7rs/definitions/ZZZZ__2a.md#definition__r7rs__ZZZZ__2a);
 * [`/`](../../r7rs/definitions/ZZZZ__2f.md#definition__r7rs__ZZZZ__2f);
 * [`expt`](../../r7rs/definitions/expt.md#definition__r7rs__expt);
 * [`square`](../../r7rs/definitions/square.md#definition__r7rs__square);
 * [`sqrt`](../../r7rs/definitions/sqrt.md#definition__r7rs__sqrt);
 * [`exp`](../../r7rs/definitions/exp.md#definition__r7rs__exp);
 * [`log`](../../r7rs/definitions/log.md#definition__r7rs__log);
 * [`sin`](../../r7rs/definitions/sin.md#definition__r7rs__sin);
 * [`cos`](../../r7rs/definitions/cos.md#definition__r7rs__cos);
 * [`tan`](../../r7rs/definitions/tan.md#definition__r7rs__tan);
 * [`asin`](../../r7rs/definitions/asin.md#definition__r7rs__asin);
 * [`acos`](../../r7rs/definitions/acos.md#definition__r7rs__acos);
 * [`atan`](../../r7rs/definitions/atan.md#definition__r7rs__atan);
 * [`abs`](../../r7rs/definitions/abs.md#definition__r7rs__abs);
 * [`floor/`](../../r7rs/definitions/floor_2f.md#definition__r7rs__floor_2f);
 * [`floor-quotient`](../../r7rs/definitions/floor-quotient.md#definition__r7rs__floor-quotient);
 * [`floor-remainder`](../../r7rs/definitions/floor-remainder.md#definition__r7rs__floor-remainder);
 * [`truncate/`](../../r7rs/definitions/truncate_2f.md#definition__r7rs__truncate_2f);
 * [`truncate-quotient`](../../r7rs/definitions/truncate-quotient.md#definition__r7rs__truncate-quotient);
 * [`truncate-remainder`](../../r7rs/definitions/truncate-remainder.md#definition__r7rs__truncate-remainder);
 * [`min`](../../r7rs/definitions/min.md#definition__r7rs__min);
 * [`max`](../../r7rs/definitions/max.md#definition__r7rs__max);

Note:  These definitions produce an output that is a sub-type.


<a id='type__r7rs__inexact-number__predicate'></a>

#### Predicate

````
(lambda (value) (and (number? value) (inexact? value)))
````


<a id='type__r7rs__inexact-number__categories'></a>

#### Categories

 * [`types-numbers`](../../r7rs/categories/types-numbers.md#category__r7rs__types-numbers);


<a id='type__r7rs__inexact-number__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

