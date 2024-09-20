n <- 1e6
x <- runif(n, -180, 180)
y <- runif(n, -90, 90)

bench::mark(
  geohash = geohash::encode(x, y, 8L),
  geohash_par = geohash::encode_par(x, y, 8L),
  geohashTools = geohashTools::gh_encode(y, x, 8L),
  iterations = 25L
)
#> # A tibble: 3 × 13
#>   expression     min  median `itr/sec` mem_alloc `gc/sec` n_itr  n_gc total_time
#>   <bch:expr> <bch:t> <bch:t>     <dbl> <bch:byt>    <dbl> <int> <dbl>   <bch:tm>
#> 1 geohash    12.53ms 12.89ms      74.2    1.54MB     0       25     0      337ms
#> 2 geohash_p…  5.41ms  5.61ms     162.   783.72KB     0       25     0      155ms
#> 3 geohashTo…  8.79ms  9.08ms     108.   816.73KB     4.49    24     1      223ms
