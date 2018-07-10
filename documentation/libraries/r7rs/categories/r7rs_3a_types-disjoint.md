

<a id='category__r7rs__r7rs_3a_types-disjoint'></a>

# `r7rs:types-disjoint` -- `r7rs` Categories


#### Types

[`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
[`number`](../../r7rs/types/number.md#type__r7rs__number);
[`symbol`](../../r7rs/types/symbol.md#type__r7rs__symbol);
[`character`](../../r7rs/types/character.md#type__r7rs__character);
[`string`](../../r7rs/types/string.md#type__r7rs__string);
[`bytevector`](../../r7rs/types/bytevector.md#type__r7rs__bytevector);
[`vector`](../../r7rs/types/vector.md#type__r7rs__vector);
[`null`](../../r7rs/types/null.md#type__r7rs__null);
[`pair`](../../r7rs/types/pair.md#type__r7rs__pair);
[`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
[`port`](../../r7rs/types/port.md#type__r7rs__port);
[`eof-object`](../../r7rs/types/eof-object.md#type__r7rs__eof-object);


#### Description

> ##### Disjointness of types
> 
> No object satisfies more than one of the following predicates:
> 
> ````
> boolean?          bytevector?
> char?             eof-object?
> null?             number?
> pair?             port?
> procedure?        string?
> symbol?           vector?
> ````
> 
> and all predicates created by `define-record-type`.
> 
> These predicates define the types
> __boolean__, __bytevector__, __character__, the empty list object,
> __eof-object__, __number__, __pair__, __port__, __procedure__, __string__, __symbol__, __vector__,
> and all record types.
> 
> Although there is a separate boolean type,
> any Scheme value can be used as a boolean value for the purpose of a
> conditional test.  As explained in section on booleans, all
> values count as true in such a test except for `#f`.
> This report uses the word __true__ to refer to any
> Scheme value except `#f`, and the word __false__ to refer to
> `#f`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Super-category

[`r7rs:standard-types`](../../r7rs/categories/r7rs_3a_standard-types.md#category__r7rs__r7rs_3a_standard-types);


##### Super-categories recursive

[`r7rs:standard-types`](../../r7rs/categories/r7rs_3a_standard-types.md#category__r7rs__r7rs_3a_standard-types);
[`r7rs`](../../r7rs/categories/r7rs.md#category__r7rs__r7rs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

