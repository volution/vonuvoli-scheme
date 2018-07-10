

<a id='definition__r7rs__features'></a>

# `features` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|list-proper|))`
   * inputs: none;
   * output: a value of type [`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);


#### Referenced types

[`list-proper`](../../r7rs/types/list-proper.md#type__r7rs__list-proper);


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


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:evaluator`](../../r7rs/categories/vs_3a_evaluator.md#category__r7rs__vs_3a_evaluator);
[`vs:compiler`](../../r7rs/categories/vs_3a_compiler.md#category__r7rs__vs_3a_compiler);
[`vs:unsupported`](../../r7rs/categories/vs_3a_unsupported.md#category__r7rs__vs_3a_unsupported);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

