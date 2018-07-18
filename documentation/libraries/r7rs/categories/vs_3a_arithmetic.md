

<a id='category__r7rs__vs_3a_arithmetic'></a>

# `vs:arithmetic` -- `r7rs` Category


<a id='category__r7rs__vs_3a_arithmetic__definitions'></a>

#### Definitions

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
 * [`abs`](../../r7rs/definitions/abs.md#definition__r7rs__abs);
 * [`floor/`](../../r7rs/definitions/floor_2f.md#definition__r7rs__floor_2f);
 * [`floor-quotient`](../../r7rs/definitions/floor-quotient.md#definition__r7rs__floor-quotient);
 * [`floor-remainder`](../../r7rs/definitions/floor-remainder.md#definition__r7rs__floor-remainder);
 * [`truncate/`](../../r7rs/definitions/truncate_2f.md#definition__r7rs__truncate_2f);
 * [`truncate-quotient`](../../r7rs/definitions/truncate-quotient.md#definition__r7rs__truncate-quotient);
 * [`truncate-remainder`](../../r7rs/definitions/truncate-remainder.md#definition__r7rs__truncate-remainder);
 * [`floor`](../../r7rs/definitions/floor.md#definition__r7rs__floor);
 * [`ceiling`](../../r7rs/definitions/ceiling.md#definition__r7rs__ceiling);
 * [`truncate`](../../r7rs/definitions/truncate.md#definition__r7rs__truncate);
 * [`round`](../../r7rs/definitions/round.md#definition__r7rs__round);
 * [`min`](../../r7rs/definitions/min.md#definition__r7rs__min);
 * [`max`](../../r7rs/definitions/max.md#definition__r7rs__max);
 * [`gcd`](../../r7rs/definitions/gcd.md#definition__r7rs__gcd);
 * [`lcm`](../../r7rs/definitions/lcm.md#definition__r7rs__lcm);
 * [`expt`](../../r7rs/definitions/expt.md#definition__r7rs__expt);
 * [`square`](../../r7rs/definitions/square.md#definition__r7rs__square);
 * [`exact-integer-sqrt`](../../r7rs/definitions/exact-integer-sqrt.md#definition__r7rs__exact-integer-sqrt);
 * [`rationalize`](../../r7rs/definitions/rationalize.md#definition__r7rs__rationalize);
 * [`numerator`](../../r7rs/definitions/numerator.md#definition__r7rs__numerator);
 * [`denominator`](../../r7rs/definitions/denominator.md#definition__r7rs__denominator);
 * [`inexact`](../../r7rs/definitions/inexact.md#definition__r7rs__inexact);
 * [`exact`](../../r7rs/definitions/exact.md#definition__r7rs__exact);
 * [`make-rectangular`](../../r7rs/definitions/make-rectangular.md#definition__r7rs__make-rectangular);
 * [`real-part`](../../r7rs/definitions/real-part.md#definition__r7rs__real-part);
 * [`imag-part`](../../r7rs/definitions/imag-part.md#definition__r7rs__imag-part);
 * [`make-polar`](../../r7rs/definitions/make-polar.md#definition__r7rs__make-polar);
 * [`magnitude`](../../r7rs/definitions/magnitude.md#definition__r7rs__magnitude);
 * [`angle`](../../r7rs/definitions/angle.md#definition__r7rs__angle);
 * [`sqrt`](../../r7rs/definitions/sqrt.md#definition__r7rs__sqrt);
 * [`exp`](../../r7rs/definitions/exp.md#definition__r7rs__exp);
 * [`log`](../../r7rs/definitions/log.md#definition__r7rs__log);
 * [`sin`](../../r7rs/definitions/sin.md#definition__r7rs__sin);
 * [`cos`](../../r7rs/definitions/cos.md#definition__r7rs__cos);
 * [`tan`](../../r7rs/definitions/tan.md#definition__r7rs__tan);
 * [`asin`](../../r7rs/definitions/asin.md#definition__r7rs__asin);
 * [`acos`](../../r7rs/definitions/acos.md#definition__r7rs__acos);
 * [`atan`](../../r7rs/definitions/atan.md#definition__r7rs__atan);
 * [`finite?`](../../r7rs/definitions/finite_3f.md#definition__r7rs__finite_3f);
 * [`infinite?`](../../r7rs/definitions/infinite_3f.md#definition__r7rs__infinite_3f);
 * [`nan?`](../../r7rs/definitions/nan_3f.md#definition__r7rs__nan_3f);


<a id='category__r7rs__vs_3a_arithmetic__super-categories'></a>

#### Super-categories

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

