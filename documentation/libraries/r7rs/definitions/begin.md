

<a id='definition__r7rs__begin'></a>

# `begin` -- `r7rs` Definition


<a id='definition__r7rs__begin__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__begin__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `expression`: expression;

Syntax variants:
 * `(_)`
 * `(_ expression ...)`


<a id='definition__r7rs__begin__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__begin__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__begin__description'></a>

#### Description

> ````
> (begin <expression-or-definition> ...)
> ````
> 
> 
> This form of `begin` can appear as part of a `<body>`, or at the
> outermost level of a `<program>`, or at the REPL, or directly nested
> in a `begin` that is itself of this form.
> It causes the contained expressions and definitions to be evaluated
> exactly as if the enclosing `begin` construct were not present.
> 
> **Rationale**:
> This form is commonly used in the output of
> macros (see section on macros)
> which need to generate multiple definitions and
> splice them into the context in which they are expanded.
> 
> 
> ----
> 
> 
> ````
> (begin <expression_1> <expression_2> ...)
> ````
> 
> 
> This form of `begin` can be used as an ordinary expression.
> The `<expression>`s are evaluated sequentially from left to right,
> and the values of the last `<expression>` are returned. This
> expression type is used to sequence side effects such as assignments
> or input and output.
> 
> ````
> (define x 0)
> 
> (and (= x 0)
>      (begin (set! x 5)
>             (+ x 1)))              ===>  6
> 
> (begin (display "4 plus 1 equals ")
>        (display (+ 4 1)))      ===>  #unspecified
>                   ; and prints:      4 plus 1 equals 5
> ````
> 
> Note that there is a third form of `begin` used as a library declaration:
> see section on libraries.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__begin__categories'></a>

#### Categories

 * [`vs:control`](../../r7rs/categories/vs_3a_control.md#category__r7rs__vs_3a_control);


<a id='definition__r7rs__begin__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

