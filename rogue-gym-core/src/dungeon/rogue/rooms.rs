use dungeon::Coord;
use rng::RngHandle;
use super::{Config, Room, RoomKind};
use rect_iter::{IntoTuple2, RectRange};
use tuple_map::TupleMap2;

pub(crate) fn make_room(
    is_empty: bool,
    room_size: Coord,
    upper_left: Coord,
    id: usize,
    config: &Config,
    level: u32,
    rng: &mut RngHandle,
) -> Room {
    if is_empty {
        let (x, y) = (room_size.x.0, room_size.y.0)
            .map(|size| rng.range(1..size - 1))
            .add(upper_left.into_tuple2());
        return Room::new(
            RoomKind::Empty {
                up_left: Coord::new(x, y),
            },
            true,
            id,
        );
    }
    let is_dark = rng.range(0..config.dark_level) + 1 < level;
    let kind = if is_dark && rng.does_happen(config.maze_rate_inv) {
        // maze
        RoomKind::Maze {
            range: RectRange::from_corners(upper_left, upper_left + room_size).unwrap(),
        }
    } else {
        // normal
        // normal
        let (xsize, ysize) = {
            let (xmin, ymin) = config.min_room_size.into_tuple2();
            ((room_size.x.0, xmin), (room_size.y.0, ymin)).map(|(max, min)| rng.range(min..max))
        };
        let room_range =
            RectRange::from_corners(upper_left, upper_left + Coord::new(xsize, ysize)).unwrap();
        RoomKind::Normal { range: room_range }
    };
    Room::new(kind, is_dark, id)
}

// reserved code of item generation

// floor_range = room_range - wall_range
// let floor_range = room_range.clone().slide_start((1, 1)).slide_end((1, 1));
// let floor_num = floor_range.len() as usize;
// let cleared = self.game_info.borrow().is_cleared;
// if !cleared || level >= self.config.amulet_level {
//     self.item_handle.borrow_mut().setup_for_room(
//         floor_range.clone(),
//         level,
//         |item_rc| {
//             let selected = self.rng.borrow_mut().range(0..floor_num);
//             let coord = floor_range
//                 .nth(selected)
//                 .expect("[Dungeon::gen_floor] Invalid floor_num")
//                 .into();
//             item_map.insert(coord, item_rc);
//         },
//     );
// }