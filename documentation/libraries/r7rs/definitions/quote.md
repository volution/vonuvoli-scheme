

<a id='definition__r7rs__quote'></a>

# `quote` -- `r7rs` Definitions


#### Kind

`syntax`;


#### Syntax signature

Syntax keywords:
 * `token`: value of type [any](../../r7rs/types/any.md#type__r7rs__any);

Syntax variants:
 * `(|_| |token|)`


#### Referenced types

[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (quote <datum>)
> '<datum>
> <constant>
> ````
> 
> 
> `(quote <datum>)` evaluates to `<datum>`.
> `<Datum>`
> can be any external representation of a Scheme object (see
> section on external representations).  This notation is used to include literal
> constants in Scheme code.
> 
> ````
> (quote a)                     ===>  a
> (quote #(a b c))     ===>  #(a b c)
> (quote (+ 1 2))               ===>  (+ 1 2)
> ````
> 
> `(quote <datum>)` can be abbreviated as
> `'<datum>`.  The two notations are equivalent in all
> respects.
> 
> ````
> 'a                   ===>  a
> '#(a b c)            ===>  #(a b c)
> '()                  ===>  ()
> '(+ 1 2)             ===>  (+ 1 2)
> '(quote a)           ===>  (quote a)
> ''a                  ===>  (quote a)
> ````
> 
> Numerical constants, string constants, character constants, vector
> constants, bytevector constants, and boolean constants evaluate to
> themselves; they need not be quoted.
> 
> ````
> '145932      ===>  145932
> 145932       ===>  145932
> '"abc"       ===>  "abc"
> "abc"        ===>  "abc"
> '#\space     ===>  #\space
> #\space      ===>  #\space
> '#(a 10)     ===>  #(a 10)
> #(a 10)      ===>  #(a 10)
> '#u8(64 65)  ===>  #u8(64 65)
> #u8(64 65)   ===>  #u8(64 65)
> '#t          ===>  #t
> #t           ===>  #t
> ````
> 
> As noted in section on storage model, it is an error to attempt to alter a constant
> (i.e. the value of a literal expression) using a mutation procedure like
> `set-car!` or `string-set!`.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:base`](../../r7rs/categories/r7rs_3a_base.md#category__r7rs__r7rs_3a_base);
[`vs:syntaxes`](../../r7rs/categories/vs_3a_syntaxes.md#category__r7rs__vs_3a_syntaxes);
[`vs:quotation`](../../r7rs/categories/vs_3a_quotation.md#category__r7rs__vs_3a_quotation);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

