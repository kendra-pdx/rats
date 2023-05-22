# Rats

This project is an exploration of functional typeclasses in `rust` using generic associative types. This implements The functor hierarchy between `Functor` and `Monad`, and some implementations for `Option`, `Vec`, and `Result`. There's also a `Semigroup` and `Monoid`.

 Why `rats`? I'm used to `cats` from the Scala world. So it's like `cats`, but for `rust` - hence `rats`.

 ## Learnings

- A lot of this was about learning rust through a familar lens (FP).
- `rust` has many built in idioms which alleviate similar problems that FP does.
- I have a better understanding of when to pass by borrow vs. move.
- Higher-kinded type similar programming in `rust` is close to feeling good, but not quite there.
- `rust` in some ways feels more strongly typed than OO centric languages due to an emphasis on compile time binding over dynamic dispatch polymorphism. That forces me to think through these things with a slightly different mindset.