# GMPMEE for rug

The rug-gmpmee crate provides an implementation for [rug](https://docs.rs/rug/latest/rug/) of the [GMP Modular Exponentiation Extension (GMPMEE)](https://github.com/verificatum/verificatum-gmpmee), which is a minor extension of [GMP](https://gmplib.org/). It adds simultaneous modular exponentiation and fixed base modular exponentiation functionality to the set of integer functions (the mpz-functions), as well as special purpose primality testing routines.

It contains the following implementations:
-Mmulti-exponentation (`spowm`)
- Fixed base exponentiation (`fpowm`). It contains a possibility to cache the precomputation table
- Miller-Rabin primality test

## Using rug-gmpmee

See the [gmpmee-sys](https://docs.rs/gmpmee-sys) crate.

## Licence

The rub-gmpmee crate is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. See the full text of the [LICENSE](LICENSE.md) for details.

