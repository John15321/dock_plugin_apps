// SPDX-License-Identifier: GPL-3.0-only
use gtk4::glib;
use gtk4::subclass::prelude::*;
use gtk4::Box;
use gtk4::DropTarget;
use gtk4::EventControllerMotion;
use gtk4::Revealer;
use once_cell::sync::OnceCell;

use crate::dock_list::DockList;

#[derive(Default)]
pub struct AppsContainer {
    pub saved_list: OnceCell<DockList>,
    pub active_list: OnceCell<DockList>,
}

#[glib::object_subclass]
impl ObjectSubclass for AppsContainer {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "AppsContainer";
    type Type = super::AppsContainer;
    type ParentType = gtk4::Box;
}

impl ObjectImpl for AppsContainer {}

impl WidgetImpl for AppsContainer {}

impl BoxImpl for AppsContainer {}
