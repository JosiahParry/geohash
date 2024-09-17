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

#[derive(Debug, Default, Clone, IntoDataFrameRow)]
pub struct RNeighbors {
    n: String,
    ne: String,
    e: String,
    se: String,
    s: String,
    sw: String,
    w: String,
    nw: String,
}

impl From<geohash::Neighbors> for RNeighbors {
    fn from(value: geohash::Neighbors) -> Self {
        RNeighbors {
            n: value.n,
            ne: value.ne,
            e: value.e,
            se: value.se,
            s: value.s,
            sw: value.sw,
            w: value.w,
            nw: value.nw,
        }
    }
}

#[extendr]
fn neighbors(geohash: Strings) -> Robj {
    geohash
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
        .as_robj()
        .clone()
}

extendr_module! {
    mod neighbor;
    fn neighbor;
    fn neighbors;
}
