

<a id='definition__r7rs__define-record-type'></a>

# `define-record-type` -- `r7rs` Definitions


<a id='definition__r7rs__define-record-type__kind'></a>

#### Kind

`syntax`;


<a id='definition__r7rs__define-record-type__syntax-signature'></a>

#### Syntax signature

Syntax keywords:
 * `type-identifier`: identifier;
 * `constructor-identifier`: identifier;
 * `predicate-identifier`: identifier;
 * `field-identifier`: identifier;
 * `field-accessor-identifier`: identifier;
 * `field-mutator-identifier`: identifier;
 * `constructor-descriptor`: pattern with variants:
   * `constructor-identifier`;
   * `(constructor-identifier field-identifier ...)`;
 * `field-descriptor`: pattern with variants:
   * `(field-identifier field-accessor-identifier)`;
   * `(field-identifier field-accessor-identifier field-mutator-identifier)`;

Syntax variants:
 * `(_ type-identifier constructor-descriptor predicate-identifier field-descriptor ...)`


<a id='definition__r7rs__define-record-type__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__define-record-type__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__define-record-type__description'></a>

#### Description

> ````
> (define-record-type <name>
>         <constructor> <pred> <field> ...)
> ````
> 
> 
> __Record-type definitions__ are used to introduce new data types, called
> __record types__.
> Like other definitions, they can appear either at the outermost level or in a body.
> The values of a record type are called __records__ and are
> aggregations of zero or more __fields__, each of which holds a single location.
> A predicate, a constructor, and field accessors and
> mutators are defined for each record type.
> 
> **Syntax**:
> `<name>` and `<pred>` are identifiers.
> The `<constructor>` is of the form
> ````
> (<constructor-name> <field-name> ...)
> ````
> and each `<field>` is either of the form
> ````
> (<field-name> <accessor-name>)
> ````
> or of the form
> ````
> (<field-name> <accessor-name> <modifier-name>)
> ````
> 
> It is an error for the same identifier to occur more than once as a
> field name.
> It is also an error for the same identifier to occur more than once
> as an accessor or mutator name.
> 
> The `define-record-type` construct is generative: each use creates a new record
> type that is distinct from all existing types, including Scheme's
> predefined types and other record types --- even record types of
> the same name or structure.
> 
> An instance of `define-record-type` is equivalent to the following
> definitions:
> 
>   * `<name>` is bound to a representation of the record type itself.
> This may be a run-time object or a purely syntactic representation.
> The representation is not utilized in this report, but it serves as a
> means to identify the record type for use by further language extensions.
> 
>   * `<constructor-name>` is bound to a procedure that takes as
>   many arguments as there are `<field-name>`s in the
>   `(<constructor-name> ...)` subexpression and returns a
>   new record of type `<name>`.  Fields whose names are listed with
>   `<constructor-name>` have the corresponding argument as their
>   initial value.  The initial values of all other fields are
>   unspecified.  It is an error for a field name to appear in
>   `<constructor>` but not as a `<field-name>`.
> 
>   * `<pred>` is bound to a predicate that returns `#t` when given a
>   value returned by the procedure bound to  `<constructor-name>` and `#f` for
>   everything else.
> 
>   * Each `<accessor-name>` is bound to a procedure that takes a record of
>   type `<name>` and returns the current value of the corresponding
>   field.  It is an error to pass an accessor a value which is not a
>   record of the appropriate type.
> 
>   * Each `<modifier-name>` is bound to a procedure that takes a record of
>   type `<name>` and a value which becomes the new value of the
>   corresponding field; an unspecified value is returned.  It is an
>   error to pass a modifier a first argument which is not a record of
>   the appropriate type.
> 
> For instance, the following record-type definition
> ````
> (define-record-type <pare>
>   (kons x y)
>   pare?
>   (x kar set-kar!)
>   (y kdr))
> ````
> defines `kons` to be a constructor, `kar` and `kdr`
> to be accessors, `set-kar!` to be a modifier, and `pare?`
> to be a predicate for instances of `<pare>`.
> 
> ````
>   (pare? (kons 1 2))        ===> #t
>   (pare? (cons 1 2))        ===> #f
>   (kar (kons 1 2))          ===> 1
>   (kdr (kons 1 2))          ===> 2
>   (let ((k (kons 1 2)))
>     (set-kar! k 3)
>     (kar k))                ===> 3
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__define-record-type__categories'></a>

#### Categories

 * [`vs:contexts`](../../r7rs/categories/vs_3a_contexts.md#category__r7rs__vs_3a_contexts);
 * [`vs:records`](../../r7rs/categories/vs_3a_records.md#category__r7rs__vs_3a_records);


<a id='definition__r7rs__define-record-type__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

