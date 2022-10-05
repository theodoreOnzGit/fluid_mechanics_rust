# Fluid Mechanics Libraries Written in Rust

## Principles

The fluid mechanics libraries help to calculate pressure losses
and mass flowrates based on the churchill friction factor 
correlation. 

The [Churchill Friction Factor Correlation](
https://powderprocess.net/Tools_html/Piping/Churchill.html)
can be written as follows for the fanning friction factor:

$$f_{darcy} = 2 \left[ (\frac{8}{Re})^{12} + 
(\frac{1}{A+B})^{1.5}\right]^{1/12}$$

Where:

$$A = \left[ 2.457 * \ln (\frac{1}{(7/Re)^{0.9}+0.27 \frac{\epsilon}{D}}
)\right]^{16}$$


$$B = \left( \frac{37530}{Re}\right)^{16}$$


