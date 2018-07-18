

<a id='definition__r7rs__cond'></a>

# `cond` -- `r7rs` Definition


<a id='definition__r7rs__cond__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__cond__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `else`: literal;
 * `condition`: expression;
 * `then-expression`: expression;
 * `clause`: pattern with variants:
   * `(condition)`;
   * `(condition then-expression ...)`;
   * `(else)`;
   * `(else then-expression ...)`;

Syntax variants:
 * `(_)`
 * `(_ clause ...)`


<a id='definition__r7rs__cond__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__cond__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__cond__description'></a>

#### Description

> ````
> (cond <clause_1> <clause_2> ...)
> else    ; auxiliary syntax
> =>      ; auxiliary syntax
> ````
> 
> 
> **Syntax**:
> `<Clauses>` take one of two forms, either
> ````
> (<test> <expression_1> ...)
> ````
> where `<test>` is any expression, or
> ````
> (<test> => <expression>)
> ````
> The last `<clause>` can be
> an "else clause", which has the form
> ````
> (else <expression_1> <expression_2> ...)
> ````
> 
> **Semantics**:
> A `cond` expression is evaluated by evaluating the `<test>`
> expressions of successive `<clause>`s in order until one of them
> evaluates to a true value (see
> section on booleans).  When a `<test>` evaluates to a true
> value, the remaining `<expression>`s in its `<clause>` are
> evaluated in order, and the results of the last `<expression>` in the
> `<clause>` are returned as the results of the entire `cond`
> expression.
> 
> If the selected `<clause>` contains only the
> `<test>` and no `<expression>`s, then the value of the
> `<test>` is returned as the result.  If the selected `<clause>` uses the
> `=>` alternate form, then the `<expression>` is evaluated.
> It is an error if its value is not a procedure that accepts one argument.  This procedure is then
> called on the value of the `<test>` and the values returned by this
> procedure are returned by the `cond` expression.
> 
> If all `<test>`s evaluate
> to `#f`, and there is no else clause, then the result of
> the conditional expression is unspecified; if there is an else
> clause, then its `<expression>`s are evaluated in order, and the values of
> the last one are returned.
> 
> ````
> (cond ((> 3 2) 'greater)
>       ((< 3 2) 'less))         ===>  greater
> 
> (cond ((> 3 3) 'greater)
>       ((< 3 3) 'less)
>       (else 'equal))            ===>  equal
> 
> (cond ((assv 'b '((a 1) (b 2))) => cadr)
>       (else #f))         ===>  2
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__cond__categories'></a>

#### Categories

 * [`vs:control`](../../r7rs/categories/vs_3a_control.md#category__r7rs__vs_3a_control);


<a id='definition__r7rs__cond__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

