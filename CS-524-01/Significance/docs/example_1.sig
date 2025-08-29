#This is an example program in significance

{x : real} # this is the `x` variable
{y : real} # this represents a change in `x`
{z : real} # z is the next iteration of `x`

x := 12.3 +/- 0.5
y := 2.6 +/- 0.2
z := x + y
z
w := x*x + y**2
w