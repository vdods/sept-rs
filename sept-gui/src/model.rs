use crate::{Action, EventHandlerCtx, ViewCtx, ViewOptions};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

#[derive(Clone, Copy, Debug)]
pub enum SaveBehavior {
    OnlyIfChanged,
    Always,
}

pub struct Model {
    /// This is the sept::dy::Value that's being viewed.  This uses Arc<RwLock<_>> so that a Value
    /// owned elsewhere can be operated upon by the Model.
    root_value_la: Arc<RwLock<sept::dy::Value>>,
    /// This is the cursor's address, indexing into the root value.
    cursor_address: sept::dy::TupleTerm,
    /// This is the queue of recorded actions.  The save_action_index and state_action_index are used to
    /// segment the queue to reason about the saved and current states of the model.
    /// - "Undo" actions those whose indices are less than current_state_action_index.
    /// - "Redo" actions those whose indices are greater than or equal to current_state_action_index.
    /// - If recorded_action_v[current_state_action_index].cumulative_root_value_edit_count ==
    /// recorded_action_v[saved_state_action_index].cumulative_root_value_edit_count, then there are
    /// no unsaved changes (to the root value; there could be changes to the cursor, but those aren't
    /// considered "changes").
    action_v: VecDeque<Action>,
    /// The queue tracks the cumulative sum of root value edits at each action.  This is used in
    /// particular to determine if there are unsaved changes.
    cumulative_root_value_edit_count_v: VecDeque<usize>,
    /// This is the index marking the first action after the saved state.
    saved_state_action_index: usize,
    /// This is the index marking the first action after the current state.
    current_state_action_index: usize,
    /// This is the path to the file that is open, if any.
    open_file_path_o: Option<std::path::PathBuf>,
    // TODO
    // local_symbol_table_la: Arc<RwLock<sept::dy::SymbolTable>>,
}

impl Model {
    pub fn new(
        root_value_la: Arc<RwLock<sept::dy::Value>>,
        cursor_address: sept::dy::TupleTerm,
        open_file_path_o: Option<std::path::PathBuf>,
    ) -> Self {
        let mut cumulative_root_value_edit_count_v = VecDeque::with_capacity(1);
        cumulative_root_value_edit_count_v.push_back(0);
        Self {
            root_value_la,
            cursor_address,
            action_v: VecDeque::new(),
            cumulative_root_value_edit_count_v,
            saved_state_action_index: 0,
            current_state_action_index: 0,
            open_file_path_o,
        }
    }
    pub fn has_unsaved_changes(&self) -> bool {
        self.cumulative_root_value_edit_count_v[self.saved_state_action_index]
            != self.cumulative_root_value_edit_count_v[self.current_state_action_index]
    }
    pub fn title(&self) -> String {
        let unsaved_changes_str = if self.has_unsaved_changes() {
            " (unsaved)"
        } else {
            ""
        };
        if let Some(open_file_path) = self.open_file_path_o.as_deref() {
            format!("SEPT - {}{}", open_file_path.display(), unsaved_changes_str)
        } else {
            format!("SEPT{}", unsaved_changes_str)
        }
    }
    pub fn cursor_address(&self) -> &sept::dy::TupleTerm {
        &self.cursor_address
    }
    pub fn open_file_path_o(&self) -> Option<&std::path::Path> {
        self.open_file_path_o.as_deref()
    }
    // TODO: Consider making this into a constructor, instead of a mutator.
    pub fn open(&mut self, path: std::path::PathBuf) {
        if self.has_unsaved_changes() {
            panic!("programmer error: Model has unsaved changes");
        }

        tracing::info!("Open: {}", path.display());
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(&path)
            .expect("TODO: handle this");

        use sept::st::Deserializable;
        let root_value = sept::dy::Value::deserialize(&mut file).expect("TODO: handle this");

        self.root_value_la = Arc::new(RwLock::new(root_value));
        // We allocate new memory for the cursor address and command queue so old memory allocations are dropped.
        self.cursor_address = sept::dy::TupleTerm::from(vec![]);
        self.action_v = VecDeque::new();
        self.cumulative_root_value_edit_count_v = VecDeque::with_capacity(1);
        self.cumulative_root_value_edit_count_v.push_back(0);
        self.saved_state_action_index = 0;
        self.current_state_action_index = 0;
        self.open_file_path_o = Some(path);
    }
    pub fn save(&mut self, save_behavior: SaveBehavior) {
        let open_file_path = self
            .open_file_path_o
            .as_deref()
            .expect("programmer error: no open file path");

        match save_behavior {
            SaveBehavior::OnlyIfChanged => {
                if !self.has_unsaved_changes() {
                    tracing::info!(
                        "Save: {} -- no unsaved changes, so not writing to disk",
                        open_file_path.display()
                    );
                    return;
                }
            }
            SaveBehavior::Always => {
                // Unconditional save.
            }
        }

        tracing::info!("Save: {}", open_file_path.display());
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&open_file_path)
            .expect("TODO: handle this");

        use sept::st::Serializable;
        self.root_value_la
            .read()
            .unwrap()
            .serialize(&mut file)
            .expect("TODO: handle this");

        self.saved_state_action_index = self.current_state_action_index;
        assert!(!self.has_unsaved_changes());
    }
    pub fn save_as(&mut self, file_path: std::path::PathBuf) {
        self.open_file_path_o = Some(file_path);
        self.save(SaveBehavior::Always);
    }
    /// Reset the model to a "blank" state, discarding any existing changes.
    pub fn clear(&mut self) {
        tracing::info!("Discard");
        use sept::dy::IntoValue;
        self.root_value_la = Arc::new(RwLock::new(sept::dy::ArrayTerm::from(vec![]).into_value()));
        self.cursor_address = sept::dy::TupleTerm::from(vec![]);
        self.action_v = VecDeque::new();
        self.cumulative_root_value_edit_count_v = VecDeque::with_capacity(1);
        self.cumulative_root_value_edit_count_v.push_back(0);
        self.saved_state_action_index = 0;
        self.current_state_action_index = 0;
        self.open_file_path_o = None;
    }
    pub fn do_action(&mut self, action: Action) -> anyhow::Result<()> {
        tracing::debug!("Do: {:?}", action);

        // Attempt to apply the action before updating the action queue, in case it fails.
        action.apply(
            &mut self.root_value_la.write().unwrap(),
            &mut self.cursor_address,
        )?;

        // Chop off the redo actions, store the applied action, and increment the current state action index.
        // TODO: Consider not chopping off the redo actions if this action is equal to the first redo action.
        self.action_v.truncate(self.current_state_action_index);
        let cumulative_root_value_edit_count =
            *self.cumulative_root_value_edit_count_v.back().unwrap()
                + action.root_value_edit_count();
        self.action_v.push_back(action);
        self.cumulative_root_value_edit_count_v
            .push_back(cumulative_root_value_edit_count);
        self.current_state_action_index += 1;

        Ok(())
    }
    pub fn undo_action(&mut self) {
        if self.current_state_action_index == 0 {
            tracing::debug!("Undo: no actions to undo");
            return;
        }

        let action = &self.action_v[self.current_state_action_index - 1];
        tracing::debug!("Undo: {:?}", action);
        action.revert(
            &mut self.root_value_la.write().unwrap(),
            &mut self.cursor_address,
        );
        self.current_state_action_index -= 1;
    }
    pub fn redo_action(&mut self) {
        if self.current_state_action_index == self.action_v.len() {
            tracing::debug!("Redo: no actions to redo");
            return;
        }

        let action = &self.action_v[self.current_state_action_index];
        tracing::info!("Redo: {:?}", action);
        action.apply(
            &mut self.root_value_la.write().unwrap(),
            &mut self.cursor_address,
        ).expect("programmer error: an action that was previously applied is failing to apply, which should not be possible");
        self.current_state_action_index += 1;
    }
    pub fn handle_event(
        &mut self,
        event: egui::Event,
        event_v: &mut VecDeque<egui::Event>,
        view_options: &ViewOptions,
    ) -> anyhow::Result<Option<egui::Event>> {
        let mut enqueued_edit_v = Vec::new();
        let unhandled_event_o = {
            let root_value_g = self.root_value_la.read().unwrap();
            let mut event_handler_ctx = EventHandlerCtx::new(
                &root_value_g,
                &self.cursor_address,
                view_options,
                event_v,
                &mut enqueued_edit_v,
            );

            use crate::EventHandler;
            root_value_g
                .handle_event(
                    event,
                    &mut event_handler_ctx,
                    &mut self.cursor_address.iter(),
                )
                .unwrap()
        };
        // Create an action out of the edits, and then apply it.
        if !enqueued_edit_v.is_empty() {
            let action = Action::new(enqueued_edit_v);
            self.do_action(action)?;
        }

        Ok(unhandled_event_o)
    }
    pub fn run_ui(&self, ui: &mut egui::Ui, view_options: &ViewOptions) {
        let old_item_spacing = ui.spacing().item_spacing;
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
        // ui.spacing_mut().item_spacing = egui::vec2(-1.0, 1.0);
        // ui.spacing_mut().item_spacing.x = 0.0;

        // This clone is so self doesn't have to be &mut.
        let mut cursor_address = self.cursor_address.clone();
        let mut view_ctx = ViewCtx::new(&self, view_options, Some(&mut cursor_address));

        ui.vertical(|ui| {
            let root_value_g = self.root_value_la.read().unwrap();
            use crate::ValueUI;
            let layout_job = root_value_g.run_ui(ui, &mut view_ctx, None);
            ui.label(layout_job);
        });

        ui.spacing_mut().item_spacing = old_item_spacing;
    }
}

impl Default for Model {
    fn default() -> Self {
        use sept::dy::IntoValue;
        // TODO: Eventually this default root value should be Placeholder or something.
        let root_value = sept::dy::ArrayTerm::from(vec![]).into_value();
        let root_value_la = Arc::new(RwLock::new(root_value));
        let cursor_address = sept::dy::TupleTerm::from(vec![]);
        Self::new(root_value_la, cursor_address, None)
    }
}
