

<a id='definition__r7rs__unless'></a>

# `unless` -- `r7rs` Definitions


<a id='definition__r7rs__unless__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__unless__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `condition`: expression;
 * `then-expression`: expression;

Syntax variants:
 * `(_ condition then-expression ...)`


<a id='definition__r7rs__unless__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__unless__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__unless__description'></a>

#### Description

> ````
> (unless <test> <expression_1> <expression_2> ...)
> ````
> 
> 
> **Syntax**:
> The `<test>` is an expression.
> 
> **Semantics**:
> The test is evaluated, and if it evaluates to `#f`,
> the expressions are evaluated in order.  The result of the `unless`
> expression is unspecified.
> 
> ````
> (unless (= 1 1.0)
>   (display "1")
>   (display "2"))  ===>  #unspecified
>            ; and prints nothing
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__unless__categories'></a>

#### Categories

 * [`vs:control`](../../r7rs/categories/vs_3a_control.md#category__r7rs__vs_3a_control);


<a id='definition__r7rs__unless__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

