
# define the R version
welfords <- function(x, new_value) {
  count <- x[1]
  mean <- x[2]
  squared_distances <- x[3]
  count <- count + 1
  delta <- new_value - mean
  new_mean <- mean + (delta / count)
  delta2 <- new_value - new_mean
  squared_distances <- squared_distances + (delta * delta2)
  sd <- sqrt(squared_distances / (count - 1))
  se <- sd / sqrt(count)
  t <- new_mean / se
  d <- new_mean / sd
  c(count, new_mean, squared_distances, sd, se, d, t)
}

list_vec_to_dataframe <- function(x, col_names) {
  n_cols <- length(x)
  x |>
    unlist() |>
    matrix(ncol = n_cols) |>
    t() |>
    as.data.frame() |>
    setNames(col_names)
}

welfords_accumulate <- function(tib) {
  purrr::accumulate(tib$value, welfords, .init = c(0, 0, 0, 0, 0, 0, 0)) |>
    list_vec_to_dataframe(c("i", "mean", "sq_dist", "sd", "se", "d", "t")) |>
    dplyr::select(-sq_dist) |>
    dplyr::mutate(group = unique(tib$group)) # add back group label
}

welfords_r <- function(input, set_size) {
tibble::tibble(
  group = rep(1:(length(input) / set_size), each = set_size),
  value = input,
) |>
  dplyr::group_by(group) |>
  dplyr::group_split() |>
  furrr::future_map(welfords_accumulate) |>
  purrr::reduce(\(x, y) rbind(x, y)) |>
  dplyr::select(group, i, mean, sd, se, d, t) |>
  magrittr::set_colnames(c("group", "n", "mean", "sd", "se", "d", "t")) |>
  dplyr::filter(n != 0)
}


# define the rust function
dyn.load("welfords.so")

welfords_rs <- function(input, set_size) {
  .Call("welfords", input, set_size)
}
input <- rnorm(10, 0.5, 1)
set_size <- 5

rust_output <- welfords_rs(input, set_size)
r_output <- welfords_r(input, set_size)

testthat::test_that("testing", {
  testthat::expect_equal(rust_output, r_output)
})
