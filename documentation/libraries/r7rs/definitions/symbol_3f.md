

<a id='definition__r7rs__symbol_3f'></a>

# `symbol?` -- `r7rs` Definitions


#### Kind

`type-predicate`;


#### Procedure signature

Procedure variants:
 * `((|symbol|) |->| (|true|))`
   * input: a value of type [`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((|any|) |->| (|false|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((|any| |...|) |->| (|boolean|))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `|vonuvoli|`


#### Referenced types

[`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
[`true`](../../r7rs/types/true.md#type__r7rs__true);
[`any`](../../r7rs/types/any.md#type__r7rs__any);
[`false`](../../r7rs/types/false.md#type__r7rs__false);
[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


#### Description

> ````
> (symbol? obj)
> ````
> 
> 
> Returns `#t` if `obj` is a symbol, otherwise returns `#f`.
> 
> ````
> (symbol? 'foo)          ===>  #t
> (symbol? (car '(a b)))  ===>  #t
> (symbol? "bar")         ===>  #f
> (symbol? 'nil)          ===>  #t
> (symbol? '())           ===>  #f
> (symbol? #f)            ===>  #f
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:symbols`](../../r7rs/categories/vs_3a_symbols.md#category__r7rs__vs_3a_symbols);
[`vs:types`](../../r7rs/categories/vs_3a_types.md#category__r7rs__vs_3a_types);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

