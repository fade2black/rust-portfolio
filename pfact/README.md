## Parallel computation of `N` factorial.

Given an integer number `N` as a command line parameter
this program computes `N` factorial.

It uses the `fork-join` parallelism to compute the factorial.
First, program detects the number of CPUs. Suppose it is 4.
Then it creates 4 independed concurrent tasks for computing the following
products:

`prod_1 = 1 x 5 x 9...`
`prod_2 = 2 x 6 x 10...`
`prod_3 = 3 x 7 x 11...`
`prod_4 = 4 x 8 x 12...`

and finally it returns `prod_1 x prod_2 x prod_3 x prod_4`.