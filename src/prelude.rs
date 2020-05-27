/*! Crate prelude

# Usage

```rust
use tear::prelude::*;
```

# Description

Simplifies importing every symbol needed for the macros to work.

It exports the following symbols:

- ValRet and its variants Val and Ret
- Judge and Return traits
- `tear!`, `terror!` and `twist!` macros
- The useful `tear_if!` and `anybox!` macros
- `next_if!` and `last_if` because they're unlikely to conflict
- (f=experimental) `impl_judge_from_try!`
*/

pub use crate::ValRet::{self, *};
pub use crate::Looping;

// Traits (needed for the macros to work)
pub use crate::Judge;
pub use crate::Return;

// Macros
pub use crate::{tear, terror, twist};
pub use crate::{tear_if, anybox};
pub use crate::{next_if, last_if};

#[cfg(feature = "experimental")] pub use crate::impl_judge_from_try;
