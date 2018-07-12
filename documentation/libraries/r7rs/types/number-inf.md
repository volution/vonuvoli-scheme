

<a id='type__r7rs__number-inf'></a>

# `number-inf` -- `r7rs` Types


#### Sub-types tree

* **[`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf)**:
  * **[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf)**;


#### Super-type

[`inexact-number-not-nan`](../../r7rs/types/inexact-number-not-nan.md#type__r7rs__inexact-number-not-nan);


##### Super-types recursive

[`inexact-number`](../../r7rs/types/inexact-number.md#type__r7rs__inexact-number);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);


#### Sub-types

[`complex-inf`](../../r7rs/types/complex-inf.md#type__r7rs__complex-inf);


##### Sub-types recursive

[`real-inf`](../../r7rs/types/real-inf.md#type__r7rs__real-inf);


#### Referent definitions as input

[`floor`](../../r7rs/definitions/floor.md#definition__r7rs__floor);
[`ceiling`](../../r7rs/definitions/ceiling.md#definition__r7rs__ceiling);
[`truncate`](../../r7rs/definitions/truncate.md#definition__r7rs__truncate);
[`round`](../../r7rs/definitions/round.md#definition__r7rs__round);
[`finite?`](../../r7rs/definitions/finite_3f.md#definition__r7rs__finite_3f);
[`infinite?`](../../r7rs/definitions/infinite_3f.md#definition__r7rs__infinite_3f);
[`nan?`](../../r7rs/definitions/nan_3f.md#definition__r7rs__nan_3f);


#### Referent definitions as input (recursive)

[`number?`](../../r7rs/definitions/number_3f.md#definition__r7rs__number_3f);
[`integer?`](../../r7rs/definitions/integer_3f.md#definition__r7rs__integer_3f);
[`real?`](../../r7rs/definitions/real_3f.md#definition__r7rs__real_3f);
[`rational?`](../../r7rs/definitions/rational_3f.md#definition__r7rs__rational_3f);
[`complex?`](../../r7rs/definitions/complex_3f.md#definition__r7rs__complex_3f);
[`exact?`](../../r7rs/definitions/exact_3f.md#definition__r7rs__exact_3f);
[`inexact?`](../../r7rs/definitions/inexact_3f.md#definition__r7rs__inexact_3f);
[`exact-integer?`](../../r7rs/definitions/exact-integer_3f.md#definition__r7rs__exact-integer_3f);
[`zero?`](../../r7rs/definitions/zero_3f.md#definition__r7rs__zero_3f);
[`positive?`](../../r7rs/definitions/positive_3f.md#definition__r7rs__positive_3f);
[`negative?`](../../r7rs/definitions/negative_3f.md#definition__r7rs__negative_3f);
[`odd?`](../../r7rs/definitions/odd_3f.md#definition__r7rs__odd_3f);
[`even?`](../../r7rs/definitions/even_3f.md#definition__r7rs__even_3f);
[`=`](../../r7rs/definitions/ZZZZ__3d.md#definition__r7rs__ZZZZ__3d);
[`<`](../../r7rs/definitions/ZZZZ__3c.md#definition__r7rs__ZZZZ__3c);
[`>`](../../r7rs/definitions/ZZZZ__3e.md#definition__r7rs__ZZZZ__3e);
[`<=`](../../r7rs/definitions/ZZZZ__3c_3d.md#definition__r7rs__ZZZZ__3c_3d);
[`>=`](../../r7rs/definitions/ZZZZ__3e_3d.md#definition__r7rs__ZZZZ__3e_3d);
[`+`](../../r7rs/definitions/ZZZZ__2b.md#definition__r7rs__ZZZZ__2b);
[`-`](../../r7rs/definitions/ZZZZ__2d.md#definition__r7rs__ZZZZ__2d);
[`*`](../../r7rs/definitions/ZZZZ__2a.md#definition__r7rs__ZZZZ__2a);
[`/`](../../r7rs/definitions/ZZZZ__2f.md#definition__r7rs__ZZZZ__2f);
[`floor/`](../../r7rs/definitions/floor_2f.md#definition__r7rs__floor_2f);
[`floor-quotient`](../../r7rs/definitions/floor-quotient.md#definition__r7rs__floor-quotient);
[`floor-remainder`](../../r7rs/definitions/floor-remainder.md#definition__r7rs__floor-remainder);
[`truncate/`](../../r7rs/definitions/truncate_2f.md#definition__r7rs__truncate_2f);
[`truncate-quotient`](../../r7rs/definitions/truncate-quotient.md#definition__r7rs__truncate-quotient);
[`truncate-remainder`](../../r7rs/definitions/truncate-remainder.md#definition__r7rs__truncate-remainder);
[`min`](../../r7rs/definitions/min.md#definition__r7rs__min);
[`max`](../../r7rs/definitions/max.md#definition__r7rs__max);
[`gcd`](../../r7rs/definitions/gcd.md#definition__r7rs__gcd);
[`lcm`](../../r7rs/definitions/lcm.md#definition__r7rs__lcm);
[`expt`](../../r7rs/definitions/expt.md#definition__r7rs__expt);
[`square`](../../r7rs/definitions/square.md#definition__r7rs__square);
[`inexact`](../../r7rs/definitions/inexact.md#definition__r7rs__inexact);
[`number->string`](../../r7rs/definitions/number-_3e_string.md#definition__r7rs__number-_3e_string);

Note:  These definitions consume an input that is a super-type.


#### Referent definitions as output

[`floor`](../../r7rs/definitions/floor.md#definition__r7rs__floor);
[`ceiling`](../../r7rs/definitions/ceiling.md#definition__r7rs__ceiling);
[`truncate`](../../r7rs/definitions/truncate.md#definition__r7rs__truncate);
[`round`](../../r7rs/definitions/round.md#definition__r7rs__round);


#### Predicate

```
(|lambda| (|value|) (|and| (|number?| |value|) (|infinite?| |value|)))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----
