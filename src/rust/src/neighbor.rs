use extendr_api::prelude::*;
use geohash::Direction;

fn as_direction(direction: &str) -> Option<Direction> {
    match direction.to_uppercase().as_str() {
        "N" => Some(Direction::N),
        "NE" => Some(Direction::NE),
        "E" => Some(Direction::E),
        "SE" => Some(Direction::SE),
        "S" => Some(Direction::S),
        "SW" => Some(Direction::SW),
        "W" => Some(Direction::W),
        "NW" => Some(Direction::NW),
        _ => None,
    }
}

fn handle_dirs(dirs: Strings, n: usize) -> Vec<Option<Direction>> {
    if dirs.len() == 1usize {
        let res = vec![as_direction(dirs[0].as_str()); n];
        return res;
    } else if dirs.len() != n {
        throw_r_error("`direction` must be a scalar or the same length as `geohash`");
    } else {
        let res = dirs
            .into_iter()
            .map(|di| {
                if di.is_na() {
                    None
                } else {
                    as_direction(di.as_str())
                }
            })
            .collect::<Vec<_>>();
        return res;
    }
}

#[extendr]
/// @export
fn neighbor(geohash: Strings, direction: Strings) -> Strings {
    let n = geohash.len();
    let dirs = handle_dirs(direction, n);

    geohash
        .into_iter()
        .zip(dirs.into_iter())
        .map(|(gi, di)| {
            if gi.is_na() | di.is_none() {
                return Rstr::na();
            }

            let resi = geohash::neighbor(gi.as_str(), di.unwrap());

            match resi {
                Ok(r) => Rstr::from(r),
                Err(_) => Rstr::na(),
            }
        })
        .collect::<Strings>()
}

#[derive(Debug, Clone, IntoDataFrameRow)]
pub struct RNeighbors {
    n: Option<String>,
    ne: Option<String>,
    e: Option<String>,
    se: Option<String>,
    s: Option<String>,
    sw: Option<String>,
    w: Option<String>,
    nw: Option<String>,
}

impl From<geohash::Neighbors> for RNeighbors {
    fn from(value: geohash::Neighbors) -> Self {
        RNeighbors {
            n: Some(value.n),
            ne: Some(value.ne),
            e: Some(value.e),
            se: Some(value.se),
            s: Some(value.s),
            sw: Some(value.sw),
            w: Some(value.w),
            nw: Some(value.nw),
        }
    }
}

impl Default for RNeighbors {
    fn default() -> Self {
        Self {
            n: None,
            ne: None,
            e: None,
            se: None,
            s: None,
            sw: None,
            w: None,
            nw: None,
        }
    }
}

#[extendr]
/// @export
fn neighbors(geohash: Strings) -> Robj {
    let mut geohash = geohash
        .into_iter()
        .map(|gi| {
            if gi.is_na() {
                RNeighbors::default()
            } else {
                match geohash::neighbors(gi.as_str()) {
                    Ok(r) => RNeighbors::from(r),
                    Err(_) => RNeighbors::default(),
                }
            }
        })
        .collect::<Vec<RNeighbors>>()
        .into_dataframe()
        .unwrap()
        .into_robj();
    geohash.set_attrib("class", ["tbl", "data.frame"]).unwrap();
    geohash
}

extendr_module! {
    mod neighbor;
    fn neighbor;
    fn neighbors;
}
