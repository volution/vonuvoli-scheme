
(list
	
	(cond)
	
	(cond (else))
	(cond (else 0))
	(cond (else 0 1))
	
	(cond (#t))
	(cond (#t 0))
	(cond (#t 0 1))
	
	(cond (#f))
	(cond (#f 0))
	(cond (#f 0 1))
	
)


(list
	
	(cond (#t) (#t))
	(cond (#f) (#f))
	(cond (#t) (#f))
	(cond (#f) (#t))
	
	(cond (#t) (#t) (else))
	(cond (#f) (#f) (else))
	(cond (#t) (#f) (else))
	(cond (#f) (#t) (else))
	
)


(list
	
	(cond (#t 1) (#t 2))
	(cond (#f 1) (#f 2))
	(cond (#t 1) (#f 2))
	(cond (#f 1) (#t 2))
	
	(cond (#t 1) (#t 2) (else 3))
	(cond (#f 1) (#f 2) (else 3))
	(cond (#t 1) (#f 2) (else 3))
	(cond (#f 1) (#t 2) (else 3))
	
)

