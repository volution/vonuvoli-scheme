

<a id='definition__r7rs__eof-object_3f'></a>

# `eof-object?` -- `r7rs` Definition


<a id='definition__r7rs__eof-object_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__eof-object_3f__extended-by'></a>

#### Extended by

 * [`eof-object?`](../../vonuvoli/definitions/eof-object_3f.md#definition__vonuvoli__eof-object_3f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__eof-object_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((eof-object) -> (true))`
   * input: a value of type [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__r7rs__eof-object_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__eof-object_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__eof-object_3f__description'></a>

#### Description

> ````
> (eof-object? obj)
> ````
> 
> 
> Returns `#t` if `obj` is an end-of-file object, otherwise returns
> `#f`.  The precise set of end-of-file objects will vary among
> implementations, but in any case no end-of-file object will ever be an object
> that can be read in using `read`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__eof-object_3f__referenced-types'></a>

#### Referenced-types

 * [`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

