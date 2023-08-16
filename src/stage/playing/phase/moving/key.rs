use std::ops::Deref;

use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Entity, Event, EventReader, EventWriter, in_state, IntoSystemConfigs, Query, Res, ResMut, Resource, Transform, With, Without};
use bevy_undo2::prelude::{AppUndoEx, UndoScheduler};

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::key::KeyBundle;
use crate::stage::playing::gimmick::player::Player;
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::phase::start_move::StartMoveEvent;
use crate::stage::state::StageState;

#[derive(Event, Debug, Copy, Clone, PartialEq)]
pub struct KeyEvent(pub Entity);


#[derive(Event, Debug, Copy, Clone, PartialEq)]
pub struct UndoKeyEvent(Vec3, PageIndex);


#[derive(Default, Resource, Debug, Copy, Clone)]
pub struct KeyCounter(usize);


impl KeyCounter {
    #[inline(always)]
    pub fn increment(&mut self) {
        self.0 += 1;
    }


    #[inline(always)]
    pub fn decrement(&mut self) {
        self.0 = self.0.checked_sub(1).unwrap_or_default();
    }
}


impl Deref for KeyCounter {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingKeyPlugin;


impl Plugin for PlayingKeyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<KeyEvent>()
            .add_undo_event::<UndoKeyEvent>()
            .init_resource::<KeyCounter>()
            .add_systems(Update, (
                key_event_system,
                undo_key_event_system
            ).run_if(in_state(StageState::Moving)));
    }
}


fn key_event_system(
    mut commands: Commands,
    mut scheduler: UndoScheduler<UndoKeyEvent>,
    mut er: EventReader<KeyEvent>,
    mut start_move_writer: EventWriter<StartMoveEvent>,
    mut key_counter: ResMut<KeyCounter>,
    player: Query<&Transform, With<Player>>,
    keys: Query<(&Transform, &PageIndex), (With<GimmickCollide>, Without<Player>)>,
) {
    for KeyEvent(ke) in er.iter().copied() {
        let Ok((kt, key_page_index)) = keys.get(ke) else { continue; };
        key_counter.increment();
        commands.entity(ke).despawn();
        start_move_writer.send(StartMoveEvent(MoveDirection::from_transform(player.single())));
        scheduler.reserve(UndoKeyEvent(kt.translation, *key_page_index));
    }
}


fn undo_key_event_system(
    mut commands: Commands,
    mut er: EventReader<UndoKeyEvent>,
    mut key_counter: ResMut<KeyCounter>,
    assets: Res<GimmickAssets>,
) {
    for UndoKeyEvent(pos, page_index) in er.iter().copied() {
        commands.spawn(KeyBundle::new(&assets, pos, page_index));
        key_counter.decrement();
    }
}