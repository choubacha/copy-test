Tests how the copy and clone trait works.

Essentially, the copy/clone derivation allows you to return a copied primative
instead of a reference. This is nice when you use a newtype that simply wraps
a primative.
