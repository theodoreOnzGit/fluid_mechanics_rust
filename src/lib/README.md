# Fluid Mechanics Libraries Written in Rust

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


