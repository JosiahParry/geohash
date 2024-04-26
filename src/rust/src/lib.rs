use extendr_api::prelude::*;
use geohash::{Coord, Rect};

#[extendr]
fn encode(x: Doubles, y: Doubles, length: i32) -> Strings {
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

    all_decoded.into_dataframe().unwrap().as_robj().clone()
}

// TODO
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
fn decode_bbox(geohash: Strings, crs: Robj) -> List {
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
}
