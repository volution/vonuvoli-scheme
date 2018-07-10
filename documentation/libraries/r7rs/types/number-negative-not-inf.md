

<a id='type__r7rs__number-negative-not-inf'></a>

# `number-negative-not-inf` -- `r7rs` Types


#### Sub-types tree

* **[`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf)**:
  * **[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative)**:
    * **[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative)**:
      * ...


#### Super-type

[`number-negative`](../../r7rs/types/number-negative.md#type__r7rs__number-negative);
[`number-negative-or-zero-not-inf`](../../r7rs/types/number-negative-or-zero-not-inf.md#type__r7rs__number-negative-or-zero-not-inf);


##### Super-types recursive

[`number-not-zero-not-nan`](../../r7rs/types/number-not-zero-not-nan.md#type__r7rs__number-not-zero-not-nan);
[`number-not-zero`](../../r7rs/types/number-not-zero.md#type__r7rs__number-not-zero);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`number-not-nan`](../../r7rs/types/number-not-nan.md#type__r7rs__number-not-nan);
[`number-negative-or-zero`](../../r7rs/types/number-negative-or-zero.md#type__r7rs__number-negative-or-zero);
[`number-not-inf`](../../r7rs/types/number-not-inf.md#type__r7rs__number-not-inf);


#### Sub-types

[`real-negative-not-inf`](../../r7rs/types/real-negative-not-inf.md#type__r7rs__real-negative-not-inf);


##### Sub-types recursive

[`rational-negative`](../../r7rs/types/rational-negative.md#type__r7rs__rational-negative);
[`integer-negative`](../../r7rs/types/integer-negative.md#type__r7rs__integer-negative);
[`exact-integer-negative`](../../r7rs/types/exact-integer-negative.md#type__r7rs__exact-integer-negative);


#### Predicate

```
(|lambda| (|value|) (|and| (|number?| |value|) (|negative?| |value|) (|not| (|infinite?| |value|))))
```


#### Categories

[`r7rs:types-numbers`](../../r7rs/categories/r7rs_3a_types-numbers.md#category__r7rs__r7rs_3a_types-numbers);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

