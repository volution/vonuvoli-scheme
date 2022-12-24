

<a id='definition__r7rs__quote'></a>

# `quote` -- `r7rs` Definition


<a id='definition__r7rs__quote__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__quote__implemented-by'></a>

#### Implemented by

 * [`quote`](../../vonuvoli/definitions/quote.md#definition__vonuvoli__quote) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__quote__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `token`: value of type [any](../../r7rs/types/any.md#type__r7rs__any);

Syntax variants:
 * `(_ token)`


<a id='definition__r7rs__quote__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__quote__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__quote__description'></a>

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


<a id='definition__r7rs__quote__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

