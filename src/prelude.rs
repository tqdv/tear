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
*/

pub use crate::ValRet::{self, *};

// Traits (needed for the macros to work)
pub use crate::Judge;
pub use crate::Return;

// Macros
pub use crate::{tear, terror, twist};
pub use crate::{tear_if, anybox};