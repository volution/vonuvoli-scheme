
(do
	(
		(timestamp-start (current-jiffy) timestamp-start)
		(loop (* 10 1000 1000) (- loop 1))
	)
	((zero? loop)
		(display (round (/ (* (- (current-jiffy) timestamp-start) 1000) (jiffies-per-second))))
		(newline))
	
	(and
		#t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t
		#t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t
		#t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t
		#t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t
		#t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t #t)
)

