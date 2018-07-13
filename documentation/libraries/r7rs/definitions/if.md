

<a id='definition__r7rs__if'></a>

# `if` -- `r7rs` Definitions


<a id='definition__r7rs__if__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__if__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `condition`: expression;
 * `then-expression`: expression;
 * `else-expression`: expression;

Syntax variants:
 * `(_ condition then-expression)`
 * `(_ condition then-expression else-expression)`


<a id='definition__r7rs__if__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__if__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__if__description'></a>

#### Description

> ````
> (if <test> <consequent> <alternate>)
> (if <test> <consequent>)
> ````
> 
> 
> **Syntax**:
> `<Test>`, `<consequent>`, and `<alternate>` are
> expressions.
> 
> **Semantics**:
> An `if` expression is evaluated as follows: first,
> `<test>` is evaluated.  If it yields a true value (see
> section on booleans), then `<consequent>` is evaluated and
> its values are returned.  Otherwise `<alternate>` is evaluated and its
> values are returned.  If `<test>` yields a false value and no
> `<alternate>` is specified, then the result of the expression is
> unspecified.
> 
> ````
> (if (> 3 2) 'yes 'no)           ===>  yes
> (if (> 2 3) 'yes 'no)           ===>  no
> (if (> 3 2)
>     (- 3 2)
>     (+ 3 2))                    ===>  1
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__if__categories'></a>

#### Categories

 * [`vs:control`](../../r7rs/categories/vs_3a_control.md#category__r7rs__vs_3a_control);


<a id='definition__r7rs__if__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

