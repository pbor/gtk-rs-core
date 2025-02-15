// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{clone, prelude::*};

use crate::{prelude::*, ActionEntry, ActionMap, SimpleAction};

pub trait ActionMapExtManual {
    #[doc(alias = "g_action_map_add_action_entries")]
    fn add_action_entries(&self, entries: impl IntoIterator<Item = ActionEntry<Self>>)
    where
        Self: IsA<ActionMap>;
}

impl<O: IsA<ActionMap>> ActionMapExtManual for O {
    fn add_action_entries(&self, entries: impl IntoIterator<Item = ActionEntry<Self>>) {
        for entry in entries.into_iter() {
            let action = if let Some(state) = entry.state() {
                SimpleAction::new_stateful(entry.name(), entry.parameter_type(), state.clone())
            } else {
                SimpleAction::new(entry.name(), entry.parameter_type())
            };
            let action_map = self.as_ref();
            if let Some(callback) = entry.activate {
                action.connect_activate(clone!(@strong action_map =>  move |action, state| {
                    // safe to unwrap as O: IsA<ActionMap>
                    callback(action_map.downcast_ref::<O>().unwrap(), action, state);
                }));
            }
            if let Some(callback) = entry.change_state {
                action.connect_change_state(clone!(@strong action_map => move |action, state| {
                    // safe to unwrap as O: IsA<ActionMap>
                    callback(action_map.downcast_ref::<O>().unwrap(), action, state);
                }));
            }
            self.as_ref().add_action(&action);
        }
    }
}
