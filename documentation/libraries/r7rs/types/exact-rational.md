

<a id='type__r7rs__exact-rational'></a>

# `exact-rational` -- `r7rs` Types


#### Sub-types tree

* **[`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer)**:
  * **[`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero)**:
    * **[`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive)**;
    * **[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative)**;
    * **[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero)**;
  * **[`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero)**:
    * **[`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero)**:
      * ...
    * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
    * **[`code-point-unicode`](../../r7rs/types/code-point-unicode.md#type__r7rs__code-point-unicode)**:
      * ...
    * **[`range-value`](../../r7rs/types/range-value.md#type__r7rs__range-value)**:
      * ...
    * **[`byte`](../../r7rs/types/byte.md#type__r7rs__byte)**:
      * ...
  * **[`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero)**:
    * [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero):
      * ...
    * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);


#### Super-type

[`exact-real`](../../r7rs/types/exact-real.md#type__r7rs__exact-real);
[`rational`](../../r7rs/types/rational.md#type__r7rs__rational);


##### Super-types recursive

[`exact-complex`](../../r7rs/types/exact-complex.md#type__r7rs__exact-complex);
[`exact-number`](../../r7rs/types/exact-number.md#type__r7rs__exact-number);
[`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
[`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
[`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);
[`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf);
[`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
[`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
[`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
[`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf);
[`real`](../../r7rs/types/real.md#type__r7rs__real);
[`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);


#### Sub-types

[`exact-integer`](../../r7rs/types/exact-integer.md#type__r7rs__exact-integer);


##### Sub-types recursive

[`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
[`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero);
[`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
[`exact-integer-positive-or-zero`](../../r7rs/types/exact-integer-positive-or-zero.md#type__r7rs__exact-integer-positive-or-zero);
[`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero);
[`code-point-unicode`](../../r7rs/types/code-point-unicode.md#type__r7rs__code-point-unicode);
[`code-point-ascii`](../../r7rs/types/code-point-ascii.md#type__r7rs__code-point-ascii);
[`range-value`](../../r7rs/types/range-value.md#type__r7rs__range-value);
[`range-offset`](../../r7rs/types/range-offset.md#type__r7rs__range-offset);
[`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
[`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
[`range-length`](../../r7rs/types/range-length.md#type__r7rs__range-length);
[`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);
[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
[`byte`](../../r7rs/types/byte.md#type__r7rs__byte);
[`byte-ascii`](../../r7rs/types/byte-ascii.md#type__r7rs__byte-ascii);


#### Predicate

```
(|lambda| (|value|) (|and| (|rational?| |value|) (|exact?| |value|)))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

