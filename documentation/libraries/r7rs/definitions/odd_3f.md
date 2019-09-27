

<a id='definition__r7rs__odd_3f'></a>

# `odd?` -- `r7rs` Definition


<a id='definition__r7rs__odd_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__odd_3f__extended-by'></a>

#### Extended by

 * [`odd?`](../../vonuvoli/definitions/odd_3f.md#definition__vonuvoli__odd_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__odd_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((integer-zero) -> (false))`
   * input: a value of type [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((integer-even) -> (false))`
   * input: a value of type [`integer-even`](../../r7rs/types/integer-even.md#type__r7rs__integer-even);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((integer-odd) -> (true))`
   * input: a value of type [`integer-odd`](../../r7rs/types/integer-odd.md#type__r7rs__integer-odd);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((integer) -> (false))`
   * input: a value of type [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__odd_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__odd_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__odd_3f__description'></a>

#### Description

> Please refer to [`zero?`](../../r7rs/definitions/zero_3f.md#definition__r7rs__zero_3f).


<a id='definition__r7rs__odd_3f__referenced-types'></a>

#### Referenced-types

 * [`integer-zero`](../../r7rs/types/integer-zero.md#type__r7rs__integer-zero);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`integer-even`](../../r7rs/types/integer-even.md#type__r7rs__integer-even);
 * [`integer-odd`](../../r7rs/types/integer-odd.md#type__r7rs__integer-odd);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`integer`](../../r7rs/types/integer.md#type__r7rs__integer);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

