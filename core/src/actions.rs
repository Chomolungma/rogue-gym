//! module for handling actions and do some operations related to multiple modules
use character::{Action, EnemyHandler, Player};
use dungeon::{Direction, Dungeon};
use error::*;
use item::{itembox::Entry as ItemEntry, ItemHandler, ItemToken};
use std::iter;
use {GameInfo, GameMsg, Reaction};

pub(crate) fn process_action(
    action: Action,
    info: &mut GameInfo,
    dungeon: &mut dyn Dungeon,
    item: &mut ItemHandler,
    player: &mut Player,
    enemies: &mut EnemyHandler,
) -> GameResult<Vec<Reaction>> {
    match action {
        Action::DownStair => {
            if dungeon.is_downstair(&player.pos) {
                new_level(info, dungeon, item, player, enemies, false)
                    .chain_err(|| "action::process_action")?;
                Ok(vec![Reaction::Redraw, Reaction::StatusUpdated])
            } else {
                Ok(vec![Reaction::Notify(GameMsg::NoDownStair)])
            }
        }
        Action::UpStair => {
            Err(ErrorId::Unimplemented.into_with(|| "UpStair Command is unimplemented"))
        }
        Action::Move(d) => Ok(move_player(d, dungeon, player, enemies)?.0),
        Action::MoveUntil(d) => {
            let mut out = Vec::new();
            loop {
                let res = move_player(d, dungeon, player, enemies)?;
                let tile = dungeon
                    .tile(&player.pos)
                    .map(|t| t.to_char())
                    .unwrap_or(' ');
                if res.1 || (tile != '.' && tile != '#') {
                    out.extend(res.0);
                    break;
                } else if out.is_empty() {
                    out.extend(res.0);
                }
            }
            Ok(out)
        }
        Action::Search => search(dungeon, player),
    }
}

pub(crate) fn move_active_enemies(
    enemies: &mut EnemyHandler,
    dungeon: &mut dyn Dungeon,
    player: &mut Player,
) -> GameResult<Vec<Reaction>> {
    let _stub = enemies.move_actives(&player.pos, None, dungeon);
    Ok(vec![])
}

pub(crate) fn new_level(
    info: &GameInfo,
    dungeon: &mut dyn Dungeon,
    item: &mut ItemHandler,
    player: &mut Player,
    enemies: &mut EnemyHandler,
    is_init: bool,
) -> GameResult<()> {
    if !is_init {
        dungeon
            .new_level(info, item, enemies)
            .chain_err(|| "action::new_level")?;
    }
    player.pos = dungeon
        .select_cell(true)
        .ok_or_else(|| ErrorId::MaybeBug.into_with(|| "action::new_level No space for player!"))?;
    dungeon.enter_room(&player.pos, enemies)
}

fn move_player(
    direction: Direction,
    dungeon: &mut dyn Dungeon,
    player: &mut Player,
    enemies: &mut EnemyHandler,
) -> GameResult<(Vec<Reaction>, bool)> {
    if !dungeon.can_move_player(&player.pos, direction) {
        return Ok((vec![Reaction::Notify(GameMsg::CantMove(direction))], true));
    }
    let new_pos = dungeon
        .move_player(&player.pos, direction, enemies)
        .chain_err(|| "actions::move_player")?;
    player.pos = new_pos;
    player.run(true);
    let mut done = false;
    let mut res = vec![Reaction::Redraw];
    if let Some(msg) = get_item(dungeon, player).chain_err(|| "in actions::move_player")? {
        res.push(Reaction::Notify(msg));
        res.push(Reaction::StatusUpdated);
        done = true;
    }
    Ok((res, done))
}

fn search(dungeon: &mut dyn Dungeon, player: &mut Player) -> GameResult<Vec<Reaction>> {
    dungeon.search(&player.pos).map(|v| {
        v.into_iter()
            .map(|msg| Reaction::Notify(msg))
            .chain(iter::once(Reaction::Redraw))
            .collect()
    })
}

fn get_item(dungeon: &mut dyn Dungeon, player: &mut Player) -> GameResult<Option<GameMsg>> {
    macro_rules! try_or_ok {
        ($res: expr) => {
            match $res {
                Some(v) => v,
                None => return Ok(None),
            }
        };
    }
    let got_item = {
        let item_ref = try_or_ok!(dungeon.get_item(&player.pos));
        let pack_entry = try_or_ok!(player.itembox.entry(item_ref));
        match pack_entry {
            ItemEntry::Insert(player_entry) => player_entry.exec(ItemToken::clone(item_ref)),
            ItemEntry::Merge(player_entry) => player_entry.exec(item_ref.get().clone()),
        }
    };
    if dungeon.remove_item(&player.pos).is_none() {
        warn!("[actions::get_item] couldn't remove object!!!")
    }
    //dungeon.remove_from_place(&player.pos);
    Ok(Some(GameMsg::GotItem {
        kind: got_item.kind.clone(),
        num: got_item.how_many.0,
    }))
}
