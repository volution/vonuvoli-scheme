

<a id='definition__r7rs__current-second'></a>

# `current-second` -- `r7rs` Definitions


<a id='definition__r7rs__current-second__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__current-second__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `(() -> (timestamp-seconds))`
   * inputs: none;
   * output: a value of type [`timestamp-seconds`](../../r7rs/types/timestamp-seconds.md#type__r7rs__timestamp-seconds);


<a id='definition__r7rs__current-second__exports'></a>

#### Exports

 * [`scheme:time`](../../r7rs/exports/scheme_3a_time.md#export__r7rs__scheme_3a_time);


<a id='definition__r7rs__current-second__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__current-second__description'></a>

#### Description

> ````
> (current-second)
> ````
> 
> 
> Returns an inexact number representing the current time on the
> __International Atomic Time (TAI)__ scale.  The value `0.0` represents midnight
> on __January 1, 1970 TAI__ (equivalent to ten seconds before midnight __Universal Time__)
> and the value `1.0` represents one __TAI__
> second later.  Neither high accuracy nor high precision are required; in particular,
> returning __Coordinated Universal Time__ plus a suitable constant might be
> the best an implementation can do.
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__current-second__referenced-types'></a>

#### Referenced-types

 * [`timestamp-seconds`](../../r7rs/types/timestamp-seconds.md#type__r7rs__timestamp-seconds);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

