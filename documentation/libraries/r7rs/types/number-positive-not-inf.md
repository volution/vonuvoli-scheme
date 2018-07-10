

<a id='type__r7rs__number-positive-not-inf'></a>

# `number-positive-not-inf` -- `r7rs` Types


#### Sub-types tree

* **[`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf)**:
  * **[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive)**:
    * **[`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive)**:
      * ...


#### Super-type

[`number-positive`](../../r7rs/types/number-positive.md#type__r7rs__number-positive);
[`number-positive-or-zero-not-inf`](../../r7rs/types/number-positive-or-zero-not-inf.md#type__r7rs__number-positive-or-zero-not-inf);


##### Super-types recursive

[`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan);
[`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
[`number-positive-or-zero`](../../r7rs/types/number-positive-or-zero.md#type__r7rs__number-positive-or-zero);
[`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);


#### Sub-types

[`real-positive-not-inf`](../../r7rs/types/real-positive-not-inf.md#type__r7rs__real-positive-not-inf);


##### Sub-types recursive

[`rational-positive`](../../r7rs/types/rational-positive.md#type__r7rs__rational-positive);
[`integer-positive`](../../r7rs/types/integer-positive.md#type__r7rs__integer-positive);
[`exact-integer-positive`](../../r7rs/types/exact-integer-positive.md#type__r7rs__exact-integer-positive);


#### Predicate

```
(|lambda| (|value|) (|and| (|number?| |value|) (|positive?| |value|) (|not| (|infinite?| |value|))))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

