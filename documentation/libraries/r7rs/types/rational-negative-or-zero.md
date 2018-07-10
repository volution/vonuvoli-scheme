

<a id='type__r7rs__rational-negative-or-zero'></a>

# `rational-negative-or-zero` -- `r7rs` Types


#### Sub-types tree

* **[`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero)**:
  * **[`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero)**:
    * **[`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero)**:
      * ...
* **[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative)**:
  * **[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative)**:
    * **[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative)**;
* **[`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero)**:
  * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero):
    * [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero):
      * ...
  * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative):
    * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
  * **[`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero)**:
    * [`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero):
      * ...
    * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);


#### Super-type

[`real-negative-or-zero-not-inf`](../../r7rs/types/real-negative-or-zero-not-inf.md#type__r7rs__real-negative-or-zero-not-inf);
[`rational`](../../r7rs/types/rational.md#type__r7rs__rational);


##### Super-types recursive

[`number-negative-or-zero-not-inf`](../../r7rs/types/number-negative-or-zero-not-inf.md#type__r7rs__number-negative-or-zero-not-inf);
[`number-negative-or-zero`](../../r7rs/types/number-negative-or-zero.md#type__r7rs__number-negative-or-zero);
[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
[`real-negative-or-zero`](../../r7rs/types/real-negative-or-zero.md#type__r7rs__real-negative-or-zero);
[`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
[`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
[`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
[`real`](../../r7rs/types/real.md#type__r7rs__real);
[`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf);
[`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf);
[`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
[`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);
[`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);


#### Sub-types

[`rational-zero`](../../r7rs/types/rational-zero.md#type__r7rs__rational-zero);
[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);
[`integer-negative-or-zero`](../../r7rs/types/integer-negative-or-zero.md#type__r7rs__integer-negative-or-zero);


##### Sub-types recursive

[`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero);
[`exact-integer-zero`](../../r7rs/types/exact-integer-zero.md#type__r7rs__exact-integer-zero);
[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
[`exact-integer-negative-or-zero`](../../r7rs/types/exact-integer-negative-or-zero.md#type__r7rs__exact-integer-negative-or-zero);
[`range-length-zero`](../../r7rs/types/range-length-zero.md#type__r7rs__range-length-zero);


#### Predicate

```
(|lambda| (|value|) (|and| (|rational?| |value|) (|or| (|negative?| |value|) (|zero?| |value|))))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

