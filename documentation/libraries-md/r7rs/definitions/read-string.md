

<a id='definition__r7rs__read-string'></a>

# `read-string` -- `r7rs` Definition


<a id='definition__r7rs__read-string__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__read-string__implemented-by'></a>

#### Implemented by

 * [`read-string`](../../vonuvoli/definitions/read-string.md#definition__vonuvoli__read-string) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__read-string__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((range-length-not-zero) -> (string-not-empty-or-eof))`
   * input: a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
   * output: a value of type [`string-not-empty-or-eof`](../../r7rs/types/string-not-empty-or-eof.md#type__r7rs__string-not-empty-or-eof);
 * `((range-length-not-zero textual-input-port-eof) -> (eof-object))`
   * inputs:
     * a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
     * a value of type [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
   * output: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * `((range-length-not-zero textual-input-port-open) -> (string-not-empty-or-eof))`
   * inputs:
     * a value of type [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
     * a value of type [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);
   * output: a value of type [`string-not-empty-or-eof`](../../r7rs/types/string-not-empty-or-eof.md#type__r7rs__string-not-empty-or-eof);


<a id='definition__r7rs__read-string__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__read-string__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__read-string__description'></a>

#### Description

> ````
> (read-string k)
> (read-string k port)
> ````
> 
> 
> Reads the next `k` characters, or as many as are available before the end of file,
> from the textual
> input `port` into a newly allocated string in left-to-right order
> and returns the string.
> If no characters are available before the end of file,
> an end-of-file object is returned.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__read-string__referenced-types'></a>

#### Referenced-types

 * [`range-length-not-zero`](../../r7rs/types/range-length-not-zero.md#type__r7rs__range-length-not-zero);
 * [`string-not-empty-or-eof`](../../r7rs/types/string-not-empty-or-eof.md#type__r7rs__string-not-empty-or-eof);
 * [`textual-input-port-eof`](../../r7rs/types/textual-input-port-eof.md#type__r7rs__textual-input-port-eof);
 * [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * [`textual-input-port-open`](../../r7rs/types/textual-input-port-open.md#type__r7rs__textual-input-port-open);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

