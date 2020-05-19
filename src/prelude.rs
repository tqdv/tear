/*! Crate prelude when implementing traits

# Usage

```rust
use tear::prelude::*;
```

# Description

It exports the following symbols:

- ValRet and its variants Val and Ret
- Moral and its variants Good and Bad
- Judge and Return traits
- tear! and rip! macros
*/

pub use crate::ValRet::{self, *};
pub use crate::Moral::{self, *};

// Traits
pub use crate::Judge;
pub use crate::Return;

// Macros
pub use crate::{tear, rip};