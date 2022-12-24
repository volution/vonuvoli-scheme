

<a id='definition__r7rs__features'></a>

# `features` -- `r7rs` Definition


<a id='definition__r7rs__features__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__features__implemented-by'></a>

#### Implemented by

 * [`features`](../../vonuvoli/definitions/features.md#definition__vonuvoli__features) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__features__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (list-proper))`
   * inputs: none;
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);


<a id='definition__r7rs__features__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__features__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__features__description'></a>

#### Description

> ````
> (features)
> ````
> 
> 
> Returns a list of the feature identifiers which `cond-expand`
> treats as true.  It is an error to modify this list.  Here is an
> example of what `features` might return:
> 
> ````
> (features) ===>
>   (r7rs ratios exact-complex full-unicode
>    gnu-linux little-endian
>    fantastic-scheme
>    fantastic-scheme-1.0
>    space-ship-control-system)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__features__referenced-types'></a>

#### Referenced-types

 * [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

