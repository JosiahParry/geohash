use std::collections::HashMap;

use extendr_api::prelude::*;
use geohash::{Coord, Rect};
mod neighbor;

use lazy_static::lazy_static;

lazy_static! {
    static ref EPSG4326: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("input", "EPSG:4326");
        m.insert("wkt", r#"GEOGCRS["WGS 84",
    ENSEMBLE["World Geodetic System 1984 ensemble",
        MEMBER["World Geodetic System 1984 (Transit)"],
        MEMBER["World Geodetic System 1984 (G730)"],
        MEMBER["World Geodetic System 1984 (G873)"],
        MEMBER["World Geodetic System 1984 (G1150)"],
        MEMBER["World Geodetic System 1984 (G1674)"],
        MEMBER["World Geodetic System 1984 (G1762)"],
        MEMBER["World Geodetic System 1984 (G2139)"],
        ELLIPSOID["WGS 84",6378137,298.257223563,
            LENGTHUNIT["metre",1]],
        ENSEMBLEACCURACY[2.0]],
    PRIMEM["Greenwich",0,
        ANGLEUNIT["degree",0.0174532925199433]],
    CS[ellipsoidal,2],
        AXIS["geodetic latitude (Lat)",north,
            ORDER[1],
            ANGLEUNIT["degree",0.0174532925199433]],
        AXIS["geodetic longitude (Lon)",east,
            ORDER[2],
            ANGLEUNIT["degree",0.0174532925199433]],
    USAGE[
        SCOPE["Horizontal component of 3D system."],
        AREA["World."],
        BBOX[-90,-180,90,180]],
    ID["EPSG",4326]]"#);
        m
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
fn encode(x: Doubles, y: Doubles, length: i32) -> Strings {

    if (length > 12i32) | (length < 1i32) {
        throw_r_error("`length` must be a value between 1 and 12")
    } else if x.len() != y.len() {
        throw_r_error("`x` and `y` must be of the same length")
    }

    x.into_iter()
        .zip(y.into_iter())
        .map(|(xi, yi)| {
            if xi.is_na() | yi.is_na() {
                Rstr::na()
            } else {
                let xi = xi.inner();
                let yi = yi.inner();
                let c = Coord { x: xi, y: yi };
                let encoded = geohash::encode(c, length as usize);

                match encoded {
                    Ok(hash) => Rstr::from(hash),
                    Err(_) => Rstr::na(),
                }
            }
        })
        .collect::<Strings>()
}

#[extendr]
/// @export
/// @rdname decode
fn decode(geohash: Strings) -> Robj {
    let all_decoded = geohash
        .into_iter()
        .map(|ghi| {
            if ghi.is_na() {
                Decoded::default()
            } else {
                let decode_raw = geohash::decode(ghi.as_str());
                match decode_raw {
                    Ok(d) => Decoded::from(d),
                    Err(_) => Default::default(),
                }
            }
        })
        .collect::<Vec<Decoded>>();

    all_decoded
        .into_dataframe()
        .unwrap()
        .as_robj()
        .set_attrib("class", ["tbl", "data.frame"])
        .unwrap()
}

#[derive(Debug, Default, Clone, IntoDataFrameRow)]
struct Decoded {
    x: Option<f64>,
    y: Option<f64>,
    x_error: Option<f64>,
    y_error: Option<f64>,
}

impl From<(Coord, f64, f64)> for Decoded {
    fn from(value: (Coord, f64, f64)) -> Self {
        Decoded {
            x: Some(value.0.x),
            y: Some(value.0.y),
            x_error: Some(value.1),
            y_error: Some(value.2),
        }
    }
}

#[extendr]
/// Decode a geohash
/// 
/// Decodes a vector of geohashes. 
/// 
/// @param geohash a character vector of geohash codes
/// @returns 
/// 
/// - `decode()` returns a `data.frame` with four columns: `x`, `y`, and `x_error`, `y_error`
/// - `decode_bbox()` returns a list of `sf` `bbox` objects
/// @export
/// @rdname decode
/// @examples 
/// decode("eyywe2zq")
/// decode_bbox("eyywe2zq")
fn decode_bbox(geohash: Strings) -> List {
    let crs = list!(
        input = EPSG4326.get("input").unwrap(),
        wkt = EPSG4326.get("wkt").unwrap()
    ).set_class(&["crs"]).unwrap();

    geohash
        .into_iter()
        .map(|ghi| {
            let hash = geohash::decode_bbox(ghi.as_str());

            match hash {
                Ok(gh) => rect_to_bbox(gh, &crs),
                Err(_) => ().into_robj(),
            }
        })
        .collect::<List>()
}

fn rect_to_bbox(x: Rect, crs: &Robj) -> Robj {
    let (xmin, ymin) = x.min().x_y();
    let (xmax, ymax) = x.max().x_y();

    let res = Doubles::from_values([xmin, ymin, xmax, ymax]);

    res.into_robj()
        .set_names(&["xmin", "ymin", "xmax", "ymax"])
        .unwrap()
        .set_attrib("crs", crs.clone())
        .unwrap()
        .set_class(&["bbox"])
        .unwrap()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod geohash;
    fn encode;
    fn decode;
    fn decode_bbox;
    use neighbor;
}
