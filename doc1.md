# Functional Programming - Traverse

## Examples

### Scala

#### Traverse an `Option` monad and apply an applicative (like `Either` which is a monad, applicative, _and_ functor) without losing the semantics of the `Option` monad that is now within the `Either` monad

```scala
// This `Traverse` is included as an implicit when importing `cats.implicits.toTraverseOps`. I've replaced the generic type `F[A]` with `Option[A]` to make it less abstract and make more sense to me 
trait Traverse[Option[_]] {
    def traverse[G[_]: Applicative, A, B](fa: Option[A])(f: A => G[B]): G[Option[B]]
}

// Assume Error is a valid type
def stringToEither(v: String): Either[Error, String] {
    Right(v)
}

val myStringOption = Some("value")

// Without traverse, we would have to do something like this:
val myEither = myStringOption
                .map(v => stringToEither(v).right.map(innerV => Some(innerV)))
                .getOrElse(Right(Option.empty))

// With traverse, we can do:
val myEither = myStringOption.traverse(stringToEither)

```

(For help understanding `monads`, `applicatives`, and `functors` see [[Functional Programming#References]])

Both solutions:

- Map the string value into an either, then from that either map the right (which is the string value) back into an option.
- If `myStringOption` was None, then `myEither` still evaluates to a `Right(None)` (and not a `Left` which is what `myStringOption.toRight(Error())` would have done).
- `myEither` only evaluates to a `Left[Error]` if `stringToEither` returned a `Left(Error())`.

This is useful in for comprehensions where the structure being mapped over is an `Either` but one of your values in the for comprehension is an `Option` that needs to have a non trivial function applied to it that results in the needed either, but you don't want that function applied if the option is `None`

## References

---
[[Functional Programming]]
<https://typelevel.org/cats/typeclasses/traverse.html>
<https://mostly-adequate.gitbook.io/mostly-adequate-guide/ch12>
