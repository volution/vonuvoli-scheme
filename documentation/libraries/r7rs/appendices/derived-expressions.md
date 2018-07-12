

<a id='appendix__r7rs__derived-expressions'></a>

# `r7rs` -- Derived expression types


<a id='appendix__r7rs__derived-expressions__description'></a>

#### Description

> This section gives syntax definitions for the derived expression types in
> terms of the primitive expression types (literal, variable, call, `lambda`,
> `if`, and `set!`), except for `quasiquote`.
> 
> 
> ##### `cond`, `case`, `and`, `when`, `unless`
> 
> ````
> (define-syntax cond
>   (syntax-rules (else =>)
>     ((cond (else result1 result2 ...))
>      (begin result1 result2 ...))
>     ((cond (test => result))
>      (let ((temp test))
>        (if temp (result temp))))
>     ((cond (test => result) clause1 clause2 ...)
>      (let ((temp test))
>        (if temp
>            (result temp)
>            (cond clause1 clause2 ...))))
>     ((cond (test)) test)
>     ((cond (test) clause1 clause2 ...)
>      (let ((temp test))
>        (if temp
>            temp
>            (cond clause1 clause2 ...))))
>     ((cond (test result1 result2 ...))
>      (if test (begin result1 result2 ...)))
>     ((cond (test result1 result2 ...)
>            clause1 clause2 ...)
>      (if test
>          (begin result1 result2 ...)
>          (cond clause1 clause2 ...)))))
> ````
> 
> ````
> (define-syntax case
>   (syntax-rules (else =>)
>     ((case (key ...)
>        clauses ...)
>      (let ((atom-key (key ...)))
>        (case atom-key clauses ...)))
>     ((case key
>        (else => result))
>      (result key))
>     ((case key
>        (else result1 result2 ...))
>      (begin result1 result2 ...))
>     ((case key
>        ((atoms ...) result1 result2 ...))
>      (if (memv key '(atoms ...))
>          (begin result1 result2 ...)))
>     ((case key
>        ((atoms ...) => result))
>      (if (memv key '(atoms ...))
>          (result key)))
>     ((case key
>        ((atoms ...) => result)
>        clause clauses ...)
>      (if (memv key '(atoms ...))
>          (result key)
>          (case key clause clauses ...)))
>     ((case key
>        ((atoms ...) result1 result2 ...)
>        clause clauses ...)
>      (if (memv key '(atoms ...))
>          (begin result1 result2 ...)
>          (case key clause clauses ...)))))
> ````
> 
> ````
> (define-syntax and
>   (syntax-rules ()
>     ((and) #t)
>     ((and test) test)
>     ((and test1 test2 ...)
>      (if test1 (and test2 ...) #f))))
> ````
> 
> ````
> (define-syntax or
>   (syntax-rules ()
>     ((or) #f)
>     ((or test) test)
>     ((or test1 test2 ...)
>      (let ((x test1))
>        (if x x (or test2 ...))))))
> ````
> 
> ````
> (define-syntax when
>   (syntax-rules ()
>     ((when test result1 result2 ...)
>      (if test
>          (begin result1 result2 ...)))))
> ````
> 
> ````
> (define-syntax unless
>   (syntax-rules ()
>     ((unless test result1 result2 ...)
>      (if (not test)
>          (begin result1 result2 ...)))))
> ````
> 
> 
> ##### `let`, `let*`, `letrec`, `letrec*`, `let-values`, `let*-values`, `define-values`
> 
> ````
> (define-syntax let
>   (syntax-rules ()
>     ((let ((name val) ...) body1 body2 ...)
>      ((lambda (name ...) body1 body2 ...)
>       val ...))
>     ((let tag ((name val) ...) body1 body2 ...)
>      ((letrec ((tag (lambda (name ...)
>                       body1 body2 ...)))
>         tag)
>       val ...))))
> ````
> 
> ````
> (define-syntax let*
>   (syntax-rules ()
>     ((let* () body1 body2 ...)
>      (let () body1 body2 ...))
>     ((let* ((name1 val1) (name2 val2) ...)
>        body1 body2 ...)
>      (let ((name1 val1))
>        (let* ((name2 val2) ...)
>          body1 body2 ...)))))
> ````
> 
> The following `letrec` macro uses the symbol `<undefined>`
> in place of an expression which returns something that when stored in
> a location makes it an error to try to obtain the value stored in the
> location.  (No such expression is defined in Scheme.)
> A trick is used to generate the temporary names needed to avoid
> specifying the order in which the values are evaluated.
> This could also be accomplished by using an auxiliary macro.
> 
> ````
> (define-syntax letrec
>   (syntax-rules ()
>     ((letrec ((var1 init1) ...) body ...)
>      (letrec "generate_temp_names"
>        (var1 ...)
>        ()
>        ((var1 init1) ...)
>        body ...))
>     ((letrec "generate_temp_names"
>        ()
>        (temp1 ...)
>        ((var1 init1) ...)
>        body ...)
>      (let ((var1 <undefined>) ...)
>        (let ((temp1 init1) ...)
>          (set! var1 temp1)
>          ...
>          body ...)))
>     ((letrec "generate_temp_names"
>        (x y ...)
>        (temp ...)
>        ((var1 init1) ...)
>        body ...)
>      (letrec "generate_temp_names"
>        (y ...)
>        (newtemp temp ...)
>        ((var1 init1) ...)
>        body ...))))
> ````
> 
> ````
> (define-syntax letrec*
>   (syntax-rules ()
>     ((letrec* ((var1 init1) ...) body1 body2 ...)
>      (let ((var1 <undefined>) ...)
>        (set! var1 init1)
>        ...
>        (let () body1 body2 ...)))))
> ````
> 
> ````
> (define-syntax let-values
>   (syntax-rules ()
>     ((let-values (binding ...) body0 body1 ...)
>      (let-values "bind"
>          (binding ...) () (begin body0 body1 ...)))
> 
>     ((let-values "bind" () tmps body)
>      (let tmps body))
> 
>     ((let-values "bind" ((b0 e0)
>          binding ...) tmps body)
>      (let-values "mktmp" b0 e0 ()
>          (binding ...) tmps body))
> 
>     ((let-values "mktmp" () e0 args
>          bindings tmps body)
>      (call-with-values
>        (lambda () e0)
>        (lambda args
>          (let-values "bind"
>              bindings tmps body))))
> 
>     ((let-values "mktmp" (a . b) e0 (arg ...)
>          bindings (tmp ...) body)
>      (let-values "mktmp" b e0 (arg ... x)
>          bindings (tmp ... (a x)) body))
> 
>     ((let-values "mktmp" a e0 (arg ...)
>         bindings (tmp ...) body)
>      (call-with-values
>        (lambda () e0)
>        (lambda (arg ... . x)
>          (let-values "bind"
>              bindings (tmp ... (a x)) body))))))
> ````
> 
> ````
> (define-syntax let*-values
>   (syntax-rules ()
>     ((let*-values () body0 body1 ...)
>      (let () body0 body1 ...))
> 
>     ((let*-values (binding0 binding1 ...)
>          body0 body1 ...)
>      (let-values (binding0)
>        (let*-values (binding1 ...)
>          body0 body1 ...)))))
> ````
> 
> ````
> (define-syntax define-values
>   (syntax-rules ()
>     ((define-values () expr)
>      (define dummy
>        (call-with-values (lambda () expr)
>                          (lambda args #f))))
>     ((define-values (var) expr)
>      (define var expr))
>     ((define-values (var0 var1 ... varn) expr)
>      (begin
>        (define var0
>          (call-with-values (lambda () expr)
>                            list))
>        (define var1
>          (let ((v (cadr var0)))
>            (set-cdr! var0 (cddr var0))
>            v)) ...
>        (define varn
>          (let ((v (cadr var0)))
>            (set! var0 (car var0))
>            v))))
>     ((define-values (var0 var1 ... . varn) expr)
>      (begin
>        (define var0
>          (call-with-values (lambda () expr)
>                            list))
>        (define var1
>          (let ((v (cadr var0)))
>            (set-cdr! var0 (cddr var0))
>            v)) ...
>        (define varn
>          (let ((v (cdr var0)))
>            (set! var0 (car var0))
>            v))))
>     ((define-values var expr)
>      (define var
>        (call-with-values (lambda () expr)
>                          list)))))
> ````
> 
> 
> ##### `begin`
> 
> ````
> (define-syntax begin
>   (syntax-rules ()
>     ((begin exp ...)
>      ((lambda () exp ...)))))
> ````
> 
> The following alternative expansion for `begin` does not make use of
> the ability to write more than one expression in the body of a lambda
> expression.  In any case, note that these rules apply only if the body
> of the `begin` contains no definitions.
> 
> ````
> (define-syntax begin
>   (syntax-rules ()
>     ((begin exp)
>      exp)
>     ((begin exp1 exp2 ...)
>      (call-with-values
>          (lambda () exp1)
>        (lambda args
>          (begin exp2 ...))))))
> ````
> 
> 
> ##### `do`
> 
> The following syntax definition
> of `do` uses a trick to expand the variable clauses.
> As with `letrec` above, an auxiliary macro would also work.
> The expression `(if #f #f)` is used to obtain an unspecific
> value.
> 
> ````
> (define-syntax do
>   (syntax-rules ()
>     ((do ((var init step ...) ...)
>          (test expr ...)
>          command ...)
>      (letrec
>        ((loop
>          (lambda (var ...)
>            (if test
>                (begin
>                  (if #f #f)
>                  expr ...)
>                (begin
>                  command
>                  ...
>                  (loop (do "step" var step ...)
>                        ...))))))
>        (loop init ...)))
>     ((do "step" x)
>      x)
>     ((do "step" x y)
>      y)))
> ````
> 
> 
> ##### `delay`, `force`, `delay-force`, `make-promise`
> 
> Here is a possible implementation of `delay`, `force` and
> `delay-force`.  We define the expression
> 
> ````
> (delay-force <expression>)
> ````
> 
> to have the same meaning as the procedure call
> 
> ````
> (make-promise #f (lambda () <expression>))
> ````
> 
> as follows
> 
> ````
> (define-syntax delay-force
>   (syntax-rules ()
>     ((delay-force expression)
>      (make-promise #f (lambda () expression)))))
> ````
> 
> and we define the expression
> 
> ````
> (delay <expression>)
> ````
> 
> to have the same meaning as:
> 
> ````
> (delay-force (make-promise #t <expression>))
> ````
> 
> as follows
> 
> ````
> (define-syntax delay
>   (syntax-rules ()
>     ((delay expression)
>      (delay-force (make-promise #t expression)))))
> ````
> 
> where `make-promise` is defined as follows:
> 
> ````
> (define make-promise
>   (lambda (done? proc)
>     (list (cons done? proc))))
> ````
> 
> Finally, we define `force` to call the procedure expressions in
> promises iteratively using a trampoline technique following
> __SRFI-45__ until a non-lazy result (i.e. a value created by
> `delay` instead of `delay-force`) is returned, as follows:
> 
> ````
> (define (force promise)
>   (if (promise-done? promise)
>       (promise-value promise)
>       (let ((promise* ((promise-value promise))))
>         (unless (promise-done? promise)
>           (promise-update! promise* promise))
>         (force promise))))
> ````
> 
> with the following promise accessors:
> 
> ````
> (define promise-done?
>   (lambda (x) (car (car x))))
> (define promise-value
>   (lambda (x) (cdr (car x))))
> (define promise-update!
>   (lambda (new old)
>     (set-car! (car old) (promise-done? new))
>     (set-cdr! (car old) (promise-value new))
>     (set-car! new (car old))))
> ````
> 
> 
> ##### `make-parameter`, `parameterize`
> 
> The following implementation of `make-parameter` and
> `parameterize` is suitable for an implementation with no threads.
> Parameter objects are implemented here as procedures, using two
> arbitrary unique objects `<param-set!>` and
> `<param-convert>`:
> 
> ````
> (define (make-parameter init . o)
>   (let* ((converter
>           (if (pair? o) (car o) (lambda (x) x)))
>          (value (converter init)))
>     (lambda args
>       (cond
>        ((null? args)
>         value)
>        ((eq? (car args) <param-set!>)
>         (set! value (cadr args)))
>        ((eq? (car args) <param-convert>)
>         converter)
>        (else
>         (error "bad parameter syntax"))))))
> ````
> 
> Then `parameterize` uses `dynamic-wind` to dynamically rebind
> the associated value:
> 
> ````
> (define-syntax parameterize
>   (syntax-rules ()
>     ((parameterize ("step")
>                    ((param value p old new) ...)
>                    ()
>                    body)
>      (let ((p param) ...)
>        (let ((old (p)) ...
>              (new ((p <param-convert>) value)) ...)
>          (dynamic-wind
>           (lambda () (p <param-set!> new) ...)
>           (lambda () . body)
>           (lambda () (p <param-set!> old) ...)))))
>     ((parameterize ("step")
>                    args
>                    ((param value) . rest)
>                    body)
>      (parameterize ("step")
>                    ((param value p old new) . args)
>                    rest
>                    body))
>     ((parameterize ((param value) ...) . body)
>      (parameterize ("step")
>                    ()
>                    ((param value) ...)
>                    body))))
> ````
> 
> 
> ##### `guard`
> 
> The following implementation of `guard` depends on an auxiliary
> macro, here called `guard-aux`.
> 
> ````
> (define-syntax guard
>   (syntax-rules ()
>     ((guard (var clause ...) e1 e2 ...)
>      ((call/cc
>        (lambda (guard-k)
>          (with-exception-handler
>           (lambda (condition)
>             ((call/cc
>                (lambda (handler-k)
>                  (guard-k
>                   (lambda ()
>                     (let ((var condition))
>                       (guard-aux
>                         (handler-k
>                           (lambda ()
>                             (raise-continuable condition)))
>                         clause ...))))))))
>           (lambda ()
>             (call-with-values
>              (lambda () e1 e2 ...)
>              (lambda args
>                (guard-k
>                  (lambda ()
>                    (apply values args)))))))))))))
> 
> (define-syntax guard-aux
>   (syntax-rules (else =>)
>     ((guard-aux reraise (else result1 result2 ...))
>      (begin result1 result2 ...))
>     ((guard-aux reraise (test => result))
>      (let ((temp test))
>        (if temp
>            (result temp)
>            reraise)))
>     ((guard-aux reraise (test => result)
>                 clause1 clause2 ...)
>      (let ((temp test))
>        (if temp
>            (result temp)
>            (guard-aux reraise clause1 clause2 ...))))
>     ((guard-aux reraise (test))
>      (or test reraise))
>     ((guard-aux reraise (test) clause1 clause2 ...)
>      (let ((temp test))
>        (if temp
>            temp
>            (guard-aux reraise clause1 clause2 ...))))
>     ((guard-aux reraise (test result1 result2 ...))
>      (if test
>          (begin result1 result2 ...)
>          reraise))
>     ((guard-aux reraise
>                 (test result1 result2 ...)
>                 clause1 clause2 ...)
>      (if test
>          (begin result1 result2 ...)
>          (guard-aux reraise clause1 clause2 ...)))))
> ````
> 
> 
> ##### `case-lambda`
> 
> ````
> (define-syntax case-lambda
>   (syntax-rules ()
>     ((case-lambda (params body0 ...) ...)
>      (lambda args
>        (let ((len (length args)))
>          (let-syntax
>              ((cl (syntax-rules ::: ()
>                     ((cl)
>                      (error "no matching clause"))
>                     ((cl ((p :::) . body) . rest)
>                      (if (= len (length '(p :::)))
>                          (apply (lambda (p :::)
>                                   . body)
>                                 args)
>                          (cl . rest)))
>                     ((cl ((p ::: . tail) . body)
>                          . rest)
>                      (if (>= len (length '(p :::)))
>                          (apply
>                           (lambda (p ::: . tail)
>                             . body)
>                           args)
>                          (cl . rest))))))
>            (cl (params body0 ...) ...)))))))
> ````
> 
> 
> ##### `cond-expand`
> 
> This definition of `cond-expand` does not interact with the
> `features` procedure.  It requires that each feature identifier provided
> by the implementation be explicitly mentioned.
> 
> ````
> (define-syntax cond-expand
>   ;; Extend this to mention all feature ids and libraries
>   (syntax-rules (and or not else r7rs library scheme base)
>     ((cond-expand)
>      (syntax-error "Unfulfilled cond-expand"))
>     ((cond-expand (else body ...))
>      (begin body ...))
>     ((cond-expand ((and) body ...) more-clauses ...)
>      (begin body ...))
>     ((cond-expand ((and req1 req2 ...) body ...)
>                   more-clauses ...)
>      (cond-expand
>        (req1
>          (cond-expand
>            ((and req2 ...) body ...)
>            more-clauses ...))
>        more-clauses ...))
>     ((cond-expand ((or) body ...) more-clauses ...)
>      (cond-expand more-clauses ...))
>     ((cond-expand ((or req1 req2 ...) body ...)
>                   more-clauses ...)
>      (cond-expand
>        (req1
>         (begin body ...))
>        (else
>         (cond-expand
>            ((or req2 ...) body ...)
>            more-clauses ...))))
>     ((cond-expand ((not req) body ...)
>                   more-clauses ...)
>      (cond-expand
>        (req
>          (cond-expand more-clauses ...))
>        (else body ...)))
>     ((cond-expand (r7rs body ...)
>                   more-clauses ...)
>        (begin body ...))
>     ;; Add clauses here for each
>     ;; supported feature identifier.
>     ;; Samples:
>     ;; ((cond-expand (exact-closed body ...)
>     ;;               more-clauses ...)
>     ;;   (begin body ...))
>     ;; ((cond-expand (ieee-float body ...)
>     ;;               more-clauses ...)
>     ;;   (begin body ...))
>     ((cond-expand ((library (scheme base))
>                    body ...)
>                   more-clauses ...)
>       (begin body ...))
>     ;; Add clauses here for each library
>     ((cond-expand (feature-id body ...)
>                   more-clauses ...)
>        (cond-expand more-clauses ...))
>     ((cond-expand ((library (name ...))
>                    body ...)
>                   more-clauses ...)
>        (cond-expand more-clauses ...))))
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----

