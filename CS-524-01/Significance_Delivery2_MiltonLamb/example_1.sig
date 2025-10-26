#This is a comment
#Below is an example program in significance

{x : real} # this is the `x` variable
{y : real} # this represents a change in `x`
{z : real} # `z` is the next iteration of `x`
{w : real} # `w` is the magnitude of `x` and `z`

x := 12.3 +/- 0.5    # assign 12.3 with uncertainty 0.5 to `x`
y := 2.6             # assign 2.6 with uncertainty 0.0 to `y`
z := x + y           # assign the sum of `x` and `y` to `z`
z                    # print `z` to console
w := x*x + z**2      # assign the sum of the square of `x` 
                     #     and the square of `z` to `w`
w
