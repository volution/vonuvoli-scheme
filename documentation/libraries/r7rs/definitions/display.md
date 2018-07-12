

<a id='definition__r7rs__display'></a>

# `display` -- `r7rs` Definitions


<a id='definition__r7rs__display__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__display__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((value) -> (void))`
   * input: a value of type [`value`](../../r7rs/types/value.md#type__r7rs__value);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * `((value textual-output-port-open) -> (void))`
   * inputs:
     * a value of type [`value`](../../r7rs/types/value.md#type__r7rs__value);
     * a value of type [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);
   * output: a value of type [`void`](../../r7rs/types/void.md#type__r7rs__void);


<a id='definition__r7rs__display__exports'></a>

#### Exports

 * [`scheme:write`](../../r7rs/exports/scheme_3a_write.md#export__r7rs__scheme_3a_write);


<a id='definition__r7rs__display__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__display__description'></a>

#### Description

> ````
> (display obj)
> (display obj port)
> ````
> 
> 
> Writes a representation of `obj` to the given textual output `port`.
> Strings that appear in the written representation are output as if by
> `write-string` instead of by `write`.
> Symbols are not escaped.  Character
> objects appear in the representation as if written by `write-char`
> instead of by `write`.
> 
> The `display` representation of other objects is unspecified.
> However, `display` must not loop forever on
> self-referencing pairs, vectors, or records.  Thus if the
> normal `write` representation is used, datum labels are needed
> to represent cycles as in `write`.
> 
> Implementations may support extended syntax to represent record types or
> other types that do not have datum representations.
> 
> The `display` procedure returns an unspecified value.
> 
> **Rationale**:  The `write` procedure is intended
> for producing machine-readable output and `display` for producing
> human-readable output.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__display__referenced-types'></a>

#### Referenced-types

 * [`value`](../../r7rs/types/value.md#type__r7rs__value);
 * [`void`](../../r7rs/types/void.md#type__r7rs__void);
 * [`textual-output-port-open`](../../r7rs/types/textual-output-port-open.md#type__r7rs__textual-output-port-open);


<a id='definition__r7rs__display__categories'></a>

#### Categories

 * [`vs:ports:output`](../../r7rs/categories/vs_3a_ports_3a_output.md#category__r7rs__vs_3a_ports_3a_output);
 * [`vs:ports:values`](../../r7rs/categories/vs_3a_ports_3a_values.md#category__r7rs__vs_3a_ports_3a_values);


<a id='definition__r7rs__display__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);
 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

