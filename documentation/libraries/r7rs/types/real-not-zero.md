

<a id='type__r7rs__real-not-zero'></a>

# `real-not-zero` -- `r7rs` Types


#### Sub-types tree

* **[`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero)**:
  * **[`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero)**:
    * **[`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero)**:
      * ...
    * **[`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive)**:
      * ...
    * **[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative)**:
      * ...
  * **[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive)**:
    * [`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive):
      * ...
  * **[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative)**:
    * [`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative):
      * ...
* **[`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan)**:
  * **[`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive)**:
    * **[`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf)**:
      * ...
  * **[`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative)**:
    * **[`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf)**:
      * ...


#### Super-type

[`complex-not-zero`](../../r7rs/types/complex-not-zero.md#type__r7rs__complex-not-zero);
[`real`](../../r7rs/types/real.md#type__r7rs__real);


##### Super-types recursive

[`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`complex`](../../r7rs/types/complex.md#type__r7rs__complex);


#### Sub-types

[`rational-not-zero`](../../r7rs/types/rational-not-zero.md#type__r7rs__rational-not-zero);
[`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan);


##### Sub-types recursive

[`integer-not-zero`](../../r7rs/types/integer-not-zero.md#type__r7rs__integer-not-zero);
[`exact-integer-not-zero`](../../r7rs/types/exact-integer-not-zero.md#type__r7rs__exact-integer-not-zero);
[`real-positive`](../../r7rs/types/real-positive.md#type__r7rs__real-positive);
[`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf);
[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive);
[`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);
[`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);
[`real-negative`](../../r7rs/types/real-negative.md#type__r7rs__real-negative);
[`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf);
[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);
[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);
[`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);


#### Predicate

```
(|lambda| (|value|) (|and| (|real?| |value|) (|not| (|zero?| |value|))))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

