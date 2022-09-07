# sqrt_rust

Incredibly simple, proof-of-concept rust program designed to determine the square root of a number. It uses the babylonian method, where you simply guess what 
the square root is. If you are correct, than radicand / guess should equal the guess (e.g. 16/4 = 4). If you are not, and are instead too low, then radicand / guess will
be larger than the answer, or if you are too high, radicand / guess will be smaller than the answer. Regardless, this means that the answer must exist somewhere between
radicand / guess and guess, so to find it you can just average the the two values to get the middle point, and try that number. If that is not correct, average the
values and try it again. If that is not correct, do it again, etc. Eventually, you will find the correct answer, where radicand / guess = guess. 

<<<<<<< Updated upstream
I'm not a super eloquent teacher, so if this did not make sense please read over [Methods of computing square roots - Wikipedia](https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method)
=======
<<<<<<< HEAD
## Known issues
Some inputs yield relatively imprecise results (e.x. 23,) because those numbers result in infinite loops, so the program will just spit out what it currently has in order to save itself.

## Contributors

Reilley Pfrimmer (b1nwal)


=======
I'm not a super eloquent teacher, so if this did not make sense please read over [Methods of computing square roots - Wikipedia](https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method)
>>>>>>> 43de60e4e7d23f5962c6f5e030f36744134295e7
>>>>>>> Stashed changes
