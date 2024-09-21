use extendr_api::prelude::*;
use rayon::prelude::*;

const MAX_K: usize = 12;
const MAP: [[u8; 8]; 4] = [*b"0145hjnp", *b"2367kmqr", *b"89destwx", *b"bcfguvyz"];

union Zd {
    d: f64,
    i: i64,
}

const MULT4: i64 = 2 << 52;
const MULT8: i64 = 3 << 52;

fn encode_single_gh<const K: usize, const LEN: usize>(xp_i: f64, yp_i: f64) -> [u8; LEN] {
    if !xp_i.is_finite() || !yp_i.is_finite() {
        panic!()
    }
    if !(-90.0..90.0).contains(&yp_i) {
        panic!()
    }

    if K > MAX_K {
        panic!()
    }

    let mut zx = Zd {
        d: if xp_i >= 180.0 {
            (xp_i + 180.0).rem_euclid(360.0) / 360.0
        } else if xp_i < -180.0 {
            1.0 + (xp_i + 180.0).rem_euclid(360.0) / 360.0
        } else {
            xp_i / 360.0 + 0.5
        },
    };
    let mut zy = Zd {
        d: yp_i / 180.0 + 0.5,
    };

    // K + 1 and last will always be null byte.
    let mut gh_elt = [0u8; LEN];

    for p in 0..K {
        unsafe {
            if p % 2 == 1 {
                zx.i += MULT4;
                zy.i += MULT8;

                let xidx = zx.d as i8;
                let yidx = zy.d as i8;

                gh_elt[p] = *MAP
                    .get_unchecked(xidx as usize)
                    .get_unchecked(yidx as usize);
            } else {
                zx.i += MULT8;
                zy.i += MULT4;

                let xidx = zx.d as i8;
                let yidx = zy.d as i8;

                gh_elt[p] = *MAP
                    .get_unchecked(yidx as usize)
                    .get_unchecked(xidx as usize);
            }

            zx.d -= (zx.d as i8) as f64;
            zy.d -= (zy.d as i8) as f64;
        }
    }

    gh_elt
}

fn encode_const<const K: usize, const LEN: usize>(longitude: &[f64], latitude: &[f64]) -> Robj {
    let n = latitude.len();

    let gh = unsafe { libR_sys::Rf_allocVector(libR_sys::SEXPTYPE::STRSXP, n as isize) };

    // This will automatically protect our sexp from gc
    let obj = Robj::from_sexp(gh);

    longitude
        .iter()
        .zip(latitude.iter())
        .enumerate()
        .for_each(|(i, (&long, &lat))| {
            let buffer = encode_single_gh::<K, LEN>(long, lat);
            let str = unsafe { libR_sys::Rf_mkChar(buffer.as_ptr() as *const i8) };
            unsafe { libR_sys::SET_STRING_ELT(gh, i as isize, str) };
        });

    obj
}

macro_rules! encode_match {
    ($fn_name:ident, $len:expr) => {
        match $len as usize {
            4 => $fn_name::<4, 5>,
            5 => $fn_name::<5, 6>,
            6 => $fn_name::<6, 7>,
            7 => $fn_name::<7, 8>,
            8 => $fn_name::<8, 9>,
            9 => $fn_name::<9, 10>,
            10 => $fn_name::<10, 11>,
            11 => $fn_name::<11, 12>,
            12 => $fn_name::<12, 13>,
            _ => panic!("Precision limit reached"),
        }
    };
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
    (encode_match!(encode_const, length))(longitude, latitude)
}

fn encode_par_const<const K: usize, const LEN: usize>(longitude: &[f64], latitude: &[f64]) -> Robj {
    let n = latitude.len();

    let gh = unsafe { libR_sys::Rf_allocVector(libR_sys::SEXPTYPE::STRSXP, n as isize) };

    // This will automatically protect our sexp from gc
    let obj = Robj::from_sexp(gh);

    let mut buffers = vec![[0u8; LEN]; n];

    buffers
        .par_iter_mut()
        .with_min_len(4096)
        .enumerate()
        .for_each(|(i, buffer)| {
            *buffer = encode_single_gh::<K, LEN>(longitude[i], latitude[i]);
        });

    for (i, buffer) in buffers.iter().enumerate() {
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
    (encode_match!(encode_par_const, length))(longitude, latitude)
}

extendr_module! {
    mod encode;
    fn encode;
    fn encode_par;
}
