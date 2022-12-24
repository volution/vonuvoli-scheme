

<a id='definition__r7rs__case'></a>

# `case` -- `r7rs` Definition


<a id='definition__r7rs__case__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__case__implemented-by'></a>

#### Implemented by

 * [`case`](../../vonuvoli/definitions/case.md#definition__vonuvoli__case) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__case__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `else`: literal;
 * `value`: expression;
 * `variant`: value of type [any](../../r7rs/types/any.md#type__r7rs__any);
 * `then-expression`: expression;
 * `clause`: pattern with variants:
   * `((variant |...|))`;
   * `((variant |...|) then-expression |...|)`;
   * `(else)`;
   * `(else then-expression |...|)`;

Syntax variants:
 * `(_ value)`
 * `(_ value clause |...|)`


<a id='definition__r7rs__case__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__case__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__case__description'></a>

#### Description

> ````
> (case <key> <clause_1> <clause_2> ...)
> else    ; auxiliary syntax
> =>      ; auxiliary syntax
> ````
> 
> **Syntax**:
> `<Key>` can be any expression.  Each `<clause>` has
> the form
> ````
> ((<datum_1> ...) <expression_1> <expression_2> ...)
> ````
> where each `<datum>` is an external representation of some object.
> It is an error if any of the `<datum>`s are the same anywhere in the expression.
> Alternatively, a `<clause>` can be of the form
> ````
> ((<datum_1> ...) => <expression>)
> ````
> The last `<clause>` can be an "else clause", which has one of the forms
> ````
> (else <expression_1> <expression_2> ...)
> ````
> or
> ````
> (else => <expression>)
> ````
> 
> **Semantics**:
> A `case` expression is evaluated as follows.  `<Key>` is
> evaluated and its result is compared against each `<datum>`.  If the
> result of evaluating `<key>` is the same (in the sense of
> `eqv?`; see section on `eqv?`) to a `<datum>`, then the
> expressions in the corresponding `<clause>` are evaluated in order
> and the results of the last expression in the `<clause>` are
> returned as the results of the `case` expression.
> 
> If the result of
> evaluating `<key>` is different from every `<datum>`, then if
> there is an else clause, its expressions are evaluated and the
> results of the last are the results of the `case` expression;
> otherwise the result of the `case` expression is unspecified.
> 
> If the selected `<clause>` or else clause uses the
> `=>` alternate form, then the `<expression>` is evaluated.
> It is an error if its value is not a procedure accepting one argument.
> This procedure is then
> called on the value of the `<key>` and the values returned by this
> procedure are returned by the `case` expression.
> 
> ````
> (case (* 2 3)
>   ((2 3 5 7) 'prime)
>   ((1 4 6 8 9) 'composite))     ===>  composite
> (case (car '(c d))
>   ((a) 'a)
>   ((b) 'b))                     ===>  #unspecified
> (case (car '(c d))
>   ((a e i o u) 'vowel)
>   ((w y) 'semivowel)
>   (else => (lambda (x) x)))     ===>  c
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__case__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

