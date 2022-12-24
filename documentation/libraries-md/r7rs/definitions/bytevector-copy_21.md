

<a id='definition__r7rs__bytevector-copy_21'></a>

# `bytevector-copy!` -- `r7rs` Definition


<a id='definition__r7rs__bytevector-copy_21__kind'></a>

#### Kind

`procedure!`;


<a id='definition__r7rs__bytevector-copy_21__implemented-by'></a>

#### Implemented by

 * [`bytevector-copy!`](../../vonuvoli/definitions/bytevector-copy_21.md#definition__vonuvoli__bytevector-copy_21) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__bytevector-copy_21__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(((source bytevector) (source-start range-start) (destination bytevector)) -> (undefined))`
   * inputs:
     * `source` of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `(((source bytevector) (source-start range-start) (destination bytevector) (destination-start range-start)) -> (undefined))`
   * inputs:
     * `source` of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * `destination-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * `(((source bytevector) (source-start range-start) (destination bytevector) (destination-start range-start) (destination-end range-end)) -> (undefined))`
   * inputs:
     * `source` of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * `source-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination` of type [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
     * `destination-start` of type [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
     * `destination-end` of type [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);
   * output: a value of type [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);


<a id='definition__r7rs__bytevector-copy_21__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__bytevector-copy_21__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__bytevector-copy_21__description'></a>

#### Description

> ````
> (bytevector-copy! to at from)
> (bytevector-copy! to at from start)
> (bytevector-copy! to at from start end)
> ````
> 
> 
> **Domain**:  It is an error if `at` is less than zero or greater than the length of `to`.
> It is also an error if `(- (bytevector-length to) at)`
> is less than `(- end start)`.
> 
> Copies the bytes of bytevector `from` between `start` and `end`
> to bytevector `to`, starting at `at`.  The order in which bytes are
> copied is unspecified, except that if the source and destination overlap,
> copying takes place as if the source is first copied into a temporary
> bytevector and then into the destination.  This can be achieved without
> allocating storage by making sure to copy in the correct direction in
> such circumstances.
> 
> ````
> (define a (bytevector 1 2 3 4 5))
> (define b (bytevector 10 20 30 40 50))
> (bytevector-copy! b 1 a 0 2)
> b ===> #u8(10 1 2 40 50)
> ````
> 
> **Note**:  This procedure appears in __R6RS__, but places the source before the destination,
> contrary to other such procedures in Scheme.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__bytevector-copy_21__referenced-types'></a>

#### Referenced-types

 * [`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
 * [`range-start`](../../r7rs/types/range-start.md#type__r7rs__range-start);
 * [`undefined`](../../r7rs/types/undefined.md#type__r7rs__undefined);
 * [`range-end`](../../r7rs/types/range-end.md#type__r7rs__range-end);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

