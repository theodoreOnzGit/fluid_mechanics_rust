# Fluid Mechanics Libraries Written in Rust

Fluid mechanics rust is a crate which helps you deal with fluid 
mechanics calculations in a unit safe manner.

The crate contains many useful traits and examples of how to use 
those traits for your own projects.

An easy way to start is to use cargo to add the fluid mechanics rust
package
```bash
cargo add fluid_mechanics_rust
```
and then start by importing the prelude
```rust
extern crate fluid_mechanics_rust;
use fluid_mechanics_rust::prelude::*;
```

Please refer to crate documentation for more details,
especially the prelude docuemntation for more examples

## Licensing

I developed this library as part of my PhD thesis and used
many free and open source libraries such as:

1. Units of measure (uom)
2. Peroxide
3. Roots

They are released under Apache 2.0 and MIT (uom and peroxide)
and roots is released under BSD 2 clause. The licensing notices
is provided in the licensing file.

## Principles

The fluid mechanics libraries help to calculate pressure losses
and mass flowrates based on the churchill friction factor 
correlation. 

The [Churchill Friction Factor Correlation](
https://powderprocess.net/Tools_html/Piping/Churchill.html)
can be written as follows for the fanning friction factor:

$$f_{fanning} = 2 \left[ \left(\frac{8}{Re}\right)^{12} + 
\left(\frac{1}{A+B}\right)^{1.5}\right]^{1/12}$$

Where:

$$A = \left[ 2.457 * \ln \left(
\frac{1}{\left(7/Re\right)^{0.9}+0.27 \frac{\epsilon}{D}}
\right)\right]^{16}$$


$$B = \left( \frac{37530}{Re}\right)^{16}$$

The darcy or moody friction factor is calculated with:

$$f_{darcy} = 4 f_{fanning}$$

The code is designed to throw an error in case of:
1. Re = 0
2. Re < 0
3. $\frac{\epsilon}{D}$ < 0


