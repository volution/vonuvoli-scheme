

<a id='type__r7rs__rational-not-zero'></a>

# `rational-not-zero` -- `r7rs` Types


#### Sub-types tree

* **[`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero)**:
  * **[`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero)**:
    * **[`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive)**;
    * **[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative)**;
    * **[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero)**;
  * **[`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive)**:
    * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
  * **[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative)**:
    * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
* **[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive)**:
  * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive):
    * [`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
* **[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative)**:
  * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative):
    * [`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);


#### Super-type

[`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero);
[`rational`](../../r7rs/types/rational.md#type__r7rs__rational);


##### Super-types recursive

[`complex-not-zero`](../../r7rs/types/complex-not-zero.md#type__r7rs__complex-not-zero);
[`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`complex`](../../r7rs/types/complex.md#type__r7rs__complex);
[`real`](../../r7rs/types/real.md#type__r7rs__real);
[`real-not-inf-not-nan`](../../r7rs/types/real-not-inf-not-nan.md#type__r7rs__real-not-inf-not-nan);
[`complex-not-inf-not-nan`](../../r7rs/types/complex-not-inf-not-nan.md#type__r7rs__complex-not-inf-not-nan);
[`number-not-inf-not-nan`](../../r7rs/types/number-not-inf-not-nan.md#type__r7rs__number-not-inf-not-nan);
[`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);
[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
[`complex-not-inf`](../../r7rs/types/complex-not-inf.md#type__r7rs__complex-not-inf);
[`complex-not-nan`](../../r7rs/types/complex-not-nan.md#type__r7rs__complex-not-nan);
[`real-not-inf`](../../r7rs/types/real-not-inf.md#type__r7rs__real-not-inf);
[`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);


#### Sub-types

[`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero);
[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive);
[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);


##### Sub-types recursive

[`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero);
[`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);
[`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);


#### Predicate

```
(|lambda| (|value|) (|and| (|rational?| |value|) (|not| (|zero?| |value|))))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

