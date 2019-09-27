

<a id='definition__r7rs__truncate_2f'></a>

# `truncate/` -- `r7rs` Definition


<a id='definition__r7rs__truncate_2f__kind'></a>

#### Kind

`procedure`;


<a id='definition__r7rs__truncate_2f__implemented-by'></a>

#### Implemented by

 * [`truncate/`](../../vonuvoli/definitions/truncate_2f.md#definition__vonuvoli__truncate_2f) (from [`vonuvoli`](../../vonuvoli/_index.md#library__vonuvoli));


<a id='definition__r7rs__truncate_2f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((real-not-nan real-not-zero-not-nan) -> (real-not-nan real-not-nan))`
   * inputs:
     * a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
     * a value of type [`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan);
   * outputs:
     * a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
     * a value of type [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * `((real real-not-zero) -> (real-nan real-nan))`
   * inputs:
     * a value of type [`real`](../../r7rs/types/real.md#type__r7rs__real);
     * a value of type [`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero);
   * outputs:
     * a value of type [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);
     * a value of type [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);


<a id='definition__r7rs__truncate_2f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__truncate_2f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__truncate_2f__description'></a>

#### Description

> Please refer to [`floor/`](../../r7rs/definitions/floor_2f.md#definition__r7rs__floor_2f).


<a id='definition__r7rs__truncate_2f__referenced-types'></a>

#### Referenced-types

 * [`real-not-nan`](../../r7rs/types/real-not-nan.md#type__r7rs__real-not-nan);
 * [`real-not-zero-not-nan`](../../r7rs/types/real-not-zero-not-nan.md#type__r7rs__real-not-zero-not-nan);
 * [`real`](../../r7rs/types/real.md#type__r7rs__real);
 * [`real-not-zero`](../../r7rs/types/real-not-zero.md#type__r7rs__real-not-zero);
 * [`real-nan`](../../r7rs/types/real-nan.md#type__r7rs__real-nan);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----

