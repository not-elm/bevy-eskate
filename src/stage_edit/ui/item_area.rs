use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{ButtonBundle, ChildBuilder, NodeBundle, PositionType, UiImage, UiRect};
use bevy::ui::{AlignItems, FlexDirection, Style, Val};
use bevy::utils::default;
use bevy_trait_query::imports::Component;
use crate::assets::gimmick::GimmickAssets;

use crate::assets::stage_edit_assets::StageEditAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::GimmickItem;
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage_edit::ui::new_gimmick_ui_image;

#[derive(Component, Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct ItemArea;


#[derive(Component, Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct ItemPlusButton;


pub fn spawn_item_area(
    parent: &mut ChildBuilder,
    gimmick_assets: &GimmickAssets,
    assets: &StageEditAssets,
    page_index: PageIndex,
    items: &[GimmickTag],
) {
    parent.spawn(
        NodeBundle {
            style: Style {
                width: Val::Px(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                padding: UiRect::top(Val::Px(18.)),
                row_gap: Val::Px(8.),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
    )
        .insert((Name::new("ItemArea"), ItemArea, page_index))
        .with_children(|parent| {
            for tag in items {
                parent
                    .spawn(new_gimmick_ui_image(*tag, gimmick_assets))
                    .insert(GimmickItem(*tag))
                    .insert(page_index);
            }
            spawn_item_plus(parent, assets, page_index);
        });
}


fn spawn_item_plus(parent: &mut ChildBuilder, assets: &StageEditAssets, page_index: PageIndex) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(32.),
            height: Val::Px(32.),
            ..default()
        },
        image: UiImage::new(assets.item_plus.clone()),
        ..default()
    })
        .insert((Name::new("ItemPlus"), ItemPlusButton, page_index));
}

