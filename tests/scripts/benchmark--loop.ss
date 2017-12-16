
(do
	(
		(timestamp-start (current-jiffy) timestamp-start)
		(loop (* 10 1000 1000) (- loop 1))
	)
	((zero? loop)
		(display (/ (* (- (current-jiffy) timestamp-start) 1000) (jiffies-per-second)))
		(newline) (display loop)
		(newline))
	
	; (cons 0 1)
)

