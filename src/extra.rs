/*! Crate prelude with all the extras

# Usage

```rust
use tear::extra::*;
```

# Description

In addition to all the symbols in `prelude`, it exports the following:

- Moral and its variants Good and Bad
- Looping
- Judge and Return traits
- Utility macros `last!`, `next!` and `resume!`
- `gut` function, and `Maru` type
*/

pub use crate::prelude::*;

// Extra types that might name conflict
pub use crate::Moral::{self, *};
pub use crate::{Judge, Return};

// Extra macros
pub use crate::{last, next, resume};

// Gutting
pub use crate::gut;
pub use crate::Maru;
