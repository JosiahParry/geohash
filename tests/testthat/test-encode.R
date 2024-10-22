test_that("encode works", {
  n <- 10
  df <- data.frame(
    y = rnorm(n, 0, 1),
    longitude  = runif(n, -180, 180),
    latitude = runif(n, -90, 90)
  )
  encoded <- encode(df$longitude, df$latitude, 5)

  expect_length(encoded, n)
  expect_type(encoded, "character")
  expect_true(nchar(encoded[[1]]) == 5)
})

