

<a id='definition__r7rs__exit'></a>

# `exit` -- `r7rs` Definitions


#### Kind

`procedure`;


#### Procedure signature

Procedure variants:
 * `(() |->| (|halt|))`
   * inputs: none;
   * output: a value of type [`halt`](../../r7rs/types/halt.md#type__r7rs__halt);
 * `((|any|) |->| (|halt|))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`halt`](../../r7rs/types/halt.md#type__r7rs__halt);


#### Referenced types

[`halt`](../../r7rs/types/halt.md#type__r7rs__halt);
[`any`](../../r7rs/types/any.md#type__r7rs__any);


#### Description

> ````
> (exit)
> (exit obj)
> ````
> 
> 
> Runs all outstanding dynamic-wind `after` procedures, terminates the
> running program, and communicates an exit value to the operating system.
> If no argument is supplied, or if `obj` is `#t`, the
> `exit` procedure should communicate to the operating system that the
> program exited normally.  If `obj` is `#f`, the `exit`
> procedure should communicate to the operating system that the program
> exited abnormally.  Otherwise, `exit` should translate `obj` into
> an appropriate exit value for the operating system, if possible.
> 
> The `exit` procedure
> must not signal an exception or return to its continuation.
> 
> **Note**:  Because of the requirement to run handlers, this procedure is not just the
> operating system's exit procedure.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


#### Categories

[`r7rs:process-context`](../../r7rs/categories/r7rs_3a_process-context.md#category__r7rs__r7rs_3a_process-context);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

