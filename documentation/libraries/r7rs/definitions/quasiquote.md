

<a id='definition__r7rs__quasiquote'></a>

# `quasiquote` -- `r7rs` Definitions


<a id='definition__r7rs__quasiquote__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__quasiquote__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `token`: value of type [any](../../r7rs/types/any.md#type__r7rs__any);

Syntax variants:
 * `(_ token)`


<a id='definition__r7rs__quasiquote__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__quasiquote__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__quasiquote__description'></a>

#### Description

> ````
> (quasiquote <qq-template>)
> `<qq-template>
> unquote            ; auxiliary syntax
> ,                  ; auxiliary syntax
> unquote-splicing   ; auxiliary syntax
> ,@                 ; auxiliary syntax
> ````
> 
> __Quasiquote__ expressions are useful
> for constructing a list or vector structure when some but not all of the
> desired structure is known in advance.  If no
> commas appear within the `<qq-template>`, the result of
> evaluating
> `$\backquote$<qq-template>` is equivalent to the result of evaluating
> `'<qq-template>`.  If a comma appears within the
> `<qq-template>`, however, the expression following the comma is
> evaluated ("unquoted") and its result is inserted into the structure
> instead of the comma and the expression.  If a comma appears followed
> without intervening whitespace by a commercial at-sign (`@`), then it is an error if the following
> expression does not evaluate to a list; the opening and closing parentheses
> of the list are then "stripped away" and the elements of the list are
> inserted in place of the comma at-sign expression sequence.  A comma
> at-sign normally appears only within a list or vector `<qq-template>`.
> 
> **Note**:
> In order to unquote an identifier beginning with `@`, it is necessary
> to use either an explicit `unquote` or to put whitespace after the comma,
> to avoid colliding with the comma at-sign sequence.
> 
> ````
> `(list ,(+ 1 2) 4)  ===>  (list 3 4)
> (let ((name 'a)) `(list ,name ',name))
>           ===>  (list a (quote a))
> `(a ,(+ 1 2) ,@(map abs '(4 -5 6)) b)
>           ===>  (a 3 4 5 6 b)
> `((foo ,(- 10 3)) ,@(cdr '(c)) . ,(car '(cons)))
>           ===>  ((foo 7) . cons)
> `#(10 5 ,(sqrt 4) ,@(map sqrt '(16 9)) 8)
>           ===>  #(10 5 2 4 3 8)
> (let ((foo '(foo bar)) (@baz 'baz))
>   `(list ,@foo , @baz))
>           ===>  (list foo bar baz)
> ````
> 
> Quasiquote expressions can be nested.  Substitutions are made only for
> unquoted components appearing at the same nesting level
> as the outermost quasiquote.  The nesting level increases by one inside
> each successive quasiquotation, and decreases by one inside each
> unquotation.
> 
> ````
> `(a `(b ,(+ 1 2) ,(foo ,(+ 1 3) d) e) f)
>           ===>  (a `(b ,(+ 1 2) ,(foo 4 d) e) f)
> (let ((name1 'x)
>       (name2 'y))
>   `(a `(b ,,name1 ,',name2 d) e))
>           ===>  (a `(b ,x ,'y d) e)
> ````
> 
> A quasiquote expression may return either newly allocated, mutable objects or
> literal structure for any structure that is constructed at run time
> during the evaluation of the expression. Portions that do not need to
> be rebuilt are always literal. Thus,
> 
> ````
> (let ((a 3)) `((1 2) ,a ,4 ,'five 6))
> ````
> 
> may be treated as equivalent to either of the following expressions:
> 
> ````
> `((1 2) 3 4 five 6)
> 
> (let ((a 3))
>   (cons '(1 2)
>         (cons a (cons 4 (cons 'five '(6))))))
> ````
> 
> However, it is not equivalent to this expression:
> 
> ````
> (let ((a 3)) (list (list 1 2) a 4 'five 6))
> ````
> 
> The two notations
> `$\backquote$<qq-template>` and `(quasiquote <qq-template>)`
> are identical in all respects.
> `,<expression>` is identical to `(unquote <expression>)`,
> and
> `,@<expression>` is identical to `(unquote-splicing <expression>)`.
> The `write` procedure may output either format.
> 
> ````
> (quasiquote (list (unquote (+ 1 2)) 4))
>           ===>  (list 3 4)
> '(quasiquote (list (unquote (+ 1 2)) 4))
>           ===>  `(list ,(+ 1 2) 4)
>       ; i.e.    (quasiquote (list (unquote (+ 1 2)) 4))
> ````
> 
> It is an error if any of the identifiers `quasiquote`, `unquote`,
> or `unquote-splicing` appear in positions within a `<qq-template>`
> otherwise than as described above.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__quasiquote__examples'></a>

#### Examples

<a id='definition__r7rs__quasiquote__example-1'></a>

##### Examples 1

 1. evaluating:
````
`(list ,(+ 1 2) 4)
````
 2. returns:
````
(list +3 +4)
````

<a id='definition__r7rs__quasiquote__example-2'></a>

##### Examples 2

 1. evaluating:
````
(let ((name 'a)) `(list ,name ',name))
````
 2. returns:
````
(list a (quote a))
````

<a id='definition__r7rs__quasiquote__example-3'></a>

##### Examples 3

 1. evaluating:
````
`(a ,(+ 1 2) ,@(map abs '(4 -5 6)) b)
````
 2. returns:
````
(a +3 +4 +5 +6 b)
````

<a id='definition__r7rs__quasiquote__example-4'></a>

##### Examples 4

 1. evaluating:
````
`((foo ,(- 10 3)) ,@(cdr '(c)) . ,(car '(cons)))
````
 2. returns:
````
((foo +7) . cons)
````

<a id='definition__r7rs__quasiquote__example-5'></a>

##### Examples 5

 1. evaluating:
````
`#(10 5 ,(sqrt 4) ,@(map sqrt '(16 9)) 8)
````
 2. returns:
````
#(+10 +5 +2 +4 +3 +8)
````

<a id='definition__r7rs__quasiquote__example-6'></a>

##### Examples 6

 1. evaluating:
````
(let ((foo '(foo bar)) (@baz 'baz))
	`(list ,@foo , @baz))
````
 2. returns:
````
(list foo bar baz)
````


<a id='definition__r7rs__quasiquote__referenced-types'></a>

#### Referenced-types

 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__r7rs__quasiquote__categories'></a>

#### Categories

 * [`vs:syntaxes`](../../r7rs/categories/vs_3a_syntaxes.md#category__r7rs__vs_3a_syntaxes);
 * [`vs:quotation`](../../r7rs/categories/vs_3a_quotation.md#category__r7rs__vs_3a_quotation);


<a id='definition__r7rs__quasiquote__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

