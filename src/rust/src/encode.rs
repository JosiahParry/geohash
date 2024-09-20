use extendr_api::prelude::*;
use rayon::prelude::*;

const MAX_K: usize = 12;

#[inline]
fn encode_single_gh(xp_i: f64, yp_i: f64, k: usize, map: &[[u8; 8]; 4]) -> [u8; MAX_K + 1] {
    if !xp_i.is_finite() || !yp_i.is_finite() {
        panic!()
    }
    if !(-90.0..90.0).contains(&yp_i) {
        panic!()
    }

    if k > MAX_K {
        panic!()
    }

    let mut zx_d = if xp_i >= 180.0 {
        (xp_i + 180.0).rem_euclid(360.0) / 360.0
    } else if xp_i < -180.0 {
        1.0 + (xp_i + 180.0).rem_euclid(360.0) / 360.0
    } else {
        xp_i / 360.0 + 0.5
    };
    let mut zy_d = yp_i / 180.0 + 0.5;

    // K + 1 and last will always be null byte.
    let mut gh_elt = [0u8; MAX_K + 1];

    (0..k).for_each(|p| {
        if p % 2 == 1 {
            zx_d *= 4.0;
            zy_d *= 8.0;

            let xidx = zx_d as usize;
            let yidx = zy_d as usize;

            gh_elt[p] = map[xidx][yidx];
        } else {
            zx_d *= 8.0;
            zy_d *= 4.0;

            let xidx = zx_d as usize;
            let yidx = zy_d as usize;

            gh_elt[p] = map[yidx][xidx];
        }

        zx_d -= (zx_d as usize) as f64;
        zy_d -= (zy_d as usize) as f64;
    });

    gh_elt
}

#[extendr]
/// Encode a coordinate to a geohash
///
/// Given a vector of x and y coordinates, returns the geohash of the location.
/// Coordinates must be provided in longitude and latitude. In the case that an invalid
/// longitude or latitude value is provided, an `NA` is returned and not an error.
///
/// @param x a numeric vector of longitudes. Must be within the range of [-180, 180] otherwise an `NA` will be returned.
/// @param y a numeric vector of latitudes. Must be within the range of [-90, 90] otherwise an `NA` will be returned.
/// @param length a scalar integer between the values of 1 and 12.
/// @export
fn encode(longitude: &[f64], latitude: &[f64], length: i32) -> Robj {
    let n = latitude.len();
    let k = length as usize;

    if k > MAX_K {
        panic!("Precsion limit reached");
    }

    let map: [[u8; 8]; 4] = [*b"0145hjnp", *b"2367kmqr", *b"89destwx", *b"bcfguvyz"];

    let gh = unsafe {
        libR_sys::Rf_protect(libR_sys::Rf_allocVector(
            libR_sys::SEXPTYPE::STRSXP,
            n as isize,
        ))
    };

    // This will automatically protect our sexp from gc
    let obj = Robj::from_sexp(gh);

    for i in 0..n {
        let buffer = encode_single_gh(longitude[i], latitude[i], k, &map);
        let str = unsafe { libR_sys::Rf_mkChar(buffer.as_ptr() as *const i8) };
        unsafe { libR_sys::SET_STRING_ELT(gh, i as isize, str) };
    }

    obj
}

#[extendr]
/// Encode a coordinate to a geohash
///
/// Given a vector of x and y coordinates, returns the geohash of the location.
/// Coordinates must be provided in longitude and latitude. In the case that an invalid
/// longitude or latitude value is provided, an `NA` is returned and not an error.
///
/// @param x a numeric vector of longitudes. Must be within the range of [-180, 180] otherwise an `NA` will be returned.
/// @param y a numeric vector of latitudes. Must be within the range of [-90, 90] otherwise an `NA` will be returned.
/// @param length a scalar integer between the values of 1 and 12.
/// @export
fn encode_par(longitude: &[f64], latitude: &[f64], length: i32) -> Robj {
    let n = latitude.len();
    let k = length as usize;

    if k > MAX_K {
        panic!("Precsion limit reached");
    }

    let map: [[u8; 8]; 4] = [*b"0145hjnp", *b"2367kmqr", *b"89destwx", *b"bcfguvyz"];

    let gh = unsafe {
        libR_sys::Rf_protect(libR_sys::Rf_allocVector(
            libR_sys::SEXPTYPE::STRSXP,
            n as isize,
        ))
    };

    // This will automatically protect our sexp from gc
    let obj = Robj::from_sexp(gh);

    (0..n)
        .into_par_iter()
        .map(|i| encode_single_gh(longitude[i], latitude[i], k, &map))
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
        .for_each(|(i, buffer)| {
            let str = unsafe { libR_sys::Rf_mkChar(buffer.as_ptr() as *const i8) };
            unsafe { libR_sys::SET_STRING_ELT(gh, i as isize, str) };
        });

    obj
}

extendr_module! {
    mod encode;
    fn encode;
    fn encode_par;
}
