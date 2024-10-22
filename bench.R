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


library(recipes)
df <- data.frame(
  y = rnorm(n, 0, 1),
  longitude  = x,
  latitude = y
)

run_step_geohash  <- \(parallel) {
  rec <- df |> 
    recipe(y ~ .) |> 
    geohash::step_geohash(
      lon = longitude,
      lat = latitude,
      name = "geohash",
      options = list(length = 8L, parallel = parallel)
  )
  juiced <- rec |> prep() |> juice()
  juiced
}

run_step_mutate_par  <- \() {
  rec <- df |> 
    recipe(y ~ .) |> 
    step_mutate(
      geohash = geohash::encode_par(longitude, latitude, length = 8L)
    )
  juiced <- rec |> prep() |> juice()
  juiced
}


bench::mark(
  sequential_step = run_step_geohash(parallel = FALSE),
  parallel_step = run_step_geohash(parallel = TRUE),
  parallel_mutate = run_step_mutate_par(),
  iterations = 25L
)
# # A tibble: 3 × 13
# expression     min median `itr/sec` mem_alloc `gc/sec` n_itr  n_gc total_time
# <bch:expr>   <bch> <bch:>     <dbl> <bch:byt>    <dbl> <int> <dbl>   <bch:tm>
# 1 sequential_…  6.2s  6.43s     0.154    78.2MB   0.0597    18     7      1.95m
# 2 parallel_st… 6.13s  6.36s     0.156    46.2MB   0.0607    18     7      1.92m
# 3 parallel_mu… 6.39s   6.6s     0.151    46.2MB   0.0588    18     7      1.99m