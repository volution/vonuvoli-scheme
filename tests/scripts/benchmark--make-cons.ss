
(do
	(
		(timestamp-start (current-jiffy) timestamp-start)
		(loop (* 100 1000) (- loop 1))
	)
	((zero? loop)
		(display (/ (* (- (current-jiffy) timestamp-start) 1000) (jiffies-per-second)))
		(newline))
	
	(cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1)
	(cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1)
	(cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1)
	(cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1)
	(cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1)
	(cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1)
	(cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1)
	(cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1)
	(cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1)
	(cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1) (cons 0 1)
	
)
