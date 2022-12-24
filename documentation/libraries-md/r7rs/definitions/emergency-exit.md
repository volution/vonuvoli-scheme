

<a id='definition__r7rs__emergency-exit'></a>

# `emergency-exit` -- `r7rs` Definition


<a id='definition__r7rs__emergency-exit__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__emergency-exit__implemented-by'></a>

#### Implemented by

 * [`emergency-exit`](../../vonuvoli/definitions/emergency-exit.md#definition__vonuvoli__emergency-exit) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__emergency-exit__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (halt))`
   * inputs: none;
   * output: a value of type [`halt`](../../r7rs/types/halt.md#type__r7rs__halt);
 * `((true) -> (halt))`
   * input: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
   * output: a value of type [`halt`](../../r7rs/types/halt.md#type__r7rs__halt);
 * `((false) -> (halt))`
   * input: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
   * output: a value of type [`halt`](../../r7rs/types/halt.md#type__r7rs__halt);
 * `((any) -> (halt))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`halt`](../../r7rs/types/halt.md#type__r7rs__halt);


<a id='definition__r7rs__emergency-exit__exports'></a>

#### Exports

 * [`scheme:process-context`](../../r7rs/exports/scheme_3a_process-context.md#export__r7rs__scheme_3a_process-context) -- `(scheme process-context)`;


<a id='definition__r7rs__emergency-exit__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__emergency-exit__description'></a>

#### Description

> ````
> (emergency-exit)
> (emergency-exit obj)
> ````
> 
> 
> Terminates the program without running any
> outstanding dynamic-wind `after` procedures
> and communicates an exit value to the operating system
> in the same manner as `exit`.
> 
> **Note**:  The `emergency-exit` procedure corresponds to the `_exit` procedure
> in __Windows__ and __POSIX__.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__emergency-exit__referenced-types'></a>

#### Referenced-types

 * [`halt`](../../r7rs/types/halt.md#type__r7rs__halt);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

