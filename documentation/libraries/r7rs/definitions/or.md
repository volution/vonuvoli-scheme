

<a id='definition__r7rs__or'></a>

# `or` -- `r7rs` Definitions


<a id='definition__r7rs__or__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__or__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(_)`
 * `(_ expression ...)`


<a id='definition__r7rs__or__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__or__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__or__description'></a>

#### Description

> ````
> (or <test_1> ...)
> ````
> 
> 
> **Semantics**:
> The `<test>` expressions are evaluated from left to right, and the value of the
> first expression that evaluates to a true value (see
> section on booleans) is returned.  Any remaining expressions
> are not evaluated.  If all expressions evaluate to `#f`
> or if there are no expressions, then `#f` is returned.
> 
> ````
> (or (= 2 2) (> 2 1))            ===>  #t
> (or (= 2 2) (< 2 1))            ===>  #t
> (or #f #f #f) ===>  #f
> (or (memq 'b '(a b c))
>     (/ 3 0))                    ===>  (b c)
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__or__categories'></a>

#### Categories

 * [`vs:control`](../../r7rs/categories/vs_3a_control.md#category__r7rs__vs_3a_control);


<a id='definition__r7rs__or__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

