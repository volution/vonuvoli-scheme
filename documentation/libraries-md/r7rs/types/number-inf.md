

<a id='type__r7rs__number-inf'></a>

# `number-inf` -- `r7rs` Type


<a id='type__r7rs__number-inf__sub-types-tree'></a>

#### Sub-types tree

* **[`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf)**:
  * **[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf)**;


<a id='type__r7rs__number-inf__super-types'></a>

#### Super-types

 * [`inexact-number-not-nan`](../../r7rs/types/inexact-number-not-nan.md#type__r7rs__inexact-number-not-nan);


<a id='type__r7rs__number-inf__super-types-recursive'></a>

##### Super-types recursive

 * [`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
 * [`number`](../../r7rs/types/number.md#type__r7rs__number);
 * [`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);


<a id='type__r7rs__number-inf__sub-types'></a>

#### Sub-types

 * [`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf);


<a id='type__r7rs__number-inf__sub-types-recursive'></a>

##### Sub-types recursive

 * [`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);


<a id='type__r7rs__number-inf__referent-definitions-input'></a>

#### Referent definitions as input

 * [`finite?`](../../r7rs/definitions/finite_3f.md#definition__r7rs__finite_3f);
 * [`infinite?`](../../r7rs/definitions/infinite_3f.md#definition__r7rs__infinite_3f);
 * [`nan?`](../../r7rs/definitions/nan_3f.md#definition__r7rs__nan_3f);


<a id='type__r7rs__number-inf__referent-definitions-input-recursive'></a>

#### Referent definitions as input (recursive)

 * [`number?`](../../r7rs/definitions/number_3f.md#definition__r7rs__number_3f);
 * [`integer?`](../../r7rs/definitions/integer_3f.md#definition__r7rs__integer_3f);
 * [`real?`](../../r7rs/definitions/real_3f.md#definition__r7rs__real_3f);
 * [`rational?`](../../r7rs/definitions/rational_3f.md#definition__r7rs__rational_3f);
 * [`complex?`](../../r7rs/definitions/complex_3f.md#definition__r7rs__complex_3f);
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

Note:  These definitions consume an input that is a super-type.


<a id='type__r7rs__number-inf__predicate'></a>

#### Predicate

````
(lambda (value) (and (number? value) (infinite? value)))
````


<a id='type__r7rs__number-inf__categories'></a>

#### Categories

 * [`types-numbers`](../../r7rs/categories/types-numbers.md#category__r7rs__types-numbers);


<a id='type__r7rs__number-inf__categories-recursive'></a>

#### Categories recursive

 * [`types`](../../r7rs/categories/types.md#category__r7rs__types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

