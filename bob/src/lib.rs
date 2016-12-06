pub fn reply(phrase: &str) ->  &'static str {
  // Third attempt, reducing the type signature.
  //   the literals in the test assertions can be compiled into either a &str or String thanks to the std Into<> Trait.
  //   Both work from a signature perspective, but String is difficult in the match because it is owned, and 
  //   therefore moved into "p", which you can only do once.
  //   Even more general would be to use "Into<&str>" like we did before.

  match phrase {
    ref p if p.is_empty()          => "Fine. Be that way!",
    ref p if p.ends_with("?")      => "Sure.",
    p if p == p.to_uppercase() => "Whoa, chill out!",
    _ => "Whatever."
  }
}
