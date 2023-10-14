pub mod screenshot;
pub mod shader_utils;
pub mod ui;
pub mod utils;

pub mod drag_n_drop {
    #[allow(unused_imports)] //FIXME: after implementing texture on 3d...
    use crate::shader_utils::{YourShader, YourShader2D};
    use bevy::input::keyboard::KeyboardInput;
    use bevy::input::ButtonState;
    use bevy::prelude::*;

    use std::collections::HashMap;
    use std::path::PathBuf;

    /// Resource: Representing the index (the default texture will be at 0), and a Texture handle that we can pass to a shader
    #[derive(Default, Resource, Deref, DerefMut, Clone, Debug)]
    pub struct TexHandleQueue(pub HashMap<usize, Handle<Image>>);

    /// Event: used to store user defined textures that we allow them to drop onto the window.
    /// TX:
    /// RX:
    #[derive(Event, Deref, DerefMut, Clone, Debug)]
    pub struct UserAddedTexture(PathBuf);

    /// Supported textures extensions for the drag-n-drop your texture functionality
    /// Update this in future if we're going to support more extensions.
    static AVAILABLE_TEX_FORMATS: [&str; 3] = ["png", "jpeg", "jpg"];

    /// System: Listens for .png and .jpeg files dropped onto the window.
    pub fn file_drag_and_drop_listener(
        mut events: EventReader<FileDragAndDrop>,
        mut texture_tx: EventWriter<UserAddedTexture>,
    ) {
        events.into_iter().for_each(|event| match event {
            FileDragAndDrop::DroppedFile { path_buf, .. } => {
                texture_tx.send(UserAddedTexture(path_buf.clone()));
                debug!("TX: {:?}", event);
            }
            _ => {}
        });
    }

    /// System: Add the user's dropped file to our available textures
    pub fn add_dropped_file(
        asset_server: Res<AssetServer>,
        mut user_textures: EventReader<UserAddedTexture>,
        mut handle_queue: ResMut<TexHandleQueue>,
    ) {
        user_textures.into_iter().for_each(|tex_path| {
            if AVAILABLE_TEX_FORMATS.iter().any(|fmt| {
                tex_path
                    .extension()
                    .is_some_and(|x| x.to_string_lossy().contains(fmt))
            }) {
                debug!("RX: {:?}", tex_path.display());
            } else {
                //FIXME: remove this branch entirely, we won't need it.
                debug!("ANY FAIL ON: {:?}", tex_path.display());
            };
            let texture: Handle<Image> = asset_server.load(tex_path.as_path());

            let new_idx = handle_queue.keys().count();
            //TODO: log
            handle_queue.insert(new_idx, texture);
            debug!("New Tex @{}", new_idx);
        });
    }

    /// System: using the keys 0-9, swap the current texture to that idx.
    pub fn swap_tex_to_idx_2d(
        mut key_evr: EventReader<KeyboardInput>,
        shader_hndl: Query<&Handle<YourShader2D>>,
        mut shader_mat: ResMut<Assets<YourShader2D>>,
        user_textures: Res<TexHandleQueue>,
        // mut shader_mat_3d: ResMut<Assets<YourShader>>, // FIXME: Support this.
    ) {
        let Ok(handle) = shader_hndl.get_single() else {
            return;
        };
        if let Some(shad_mat) = shader_mat.get_mut(handle) {
            for ev in key_evr.iter() {
                // debug!("{} pressed, moving to that Tex idx.", keynum);
                match ev.state {
                    ButtonState::Pressed => {
                        match ev.key_code {
                            Some(v) => match v {
                                KeyCode::Key1 => shad_mat.set_current_tex(1),
                                KeyCode::Key2 => shad_mat.set_current_tex(2),
                                KeyCode::Key3 => shad_mat.set_current_tex(3),
                                KeyCode::Key4 => shad_mat.set_current_tex(4),
                                KeyCode::Key5 => shad_mat.set_current_tex(5),
                                KeyCode::Key6 => shad_mat.set_current_tex(6),
                                KeyCode::Key7 => shad_mat.set_current_tex(7),
                                KeyCode::Key8 => shad_mat.set_current_tex(8),
                                KeyCode::Key9 => shad_mat.set_current_tex(9),
                                KeyCode::Key0 => shad_mat.set_current_tex(0),
                                _ => return,
                            },
                            _ => return,
                        };
                    }
                    _ => return,
                }
            }
        }
    }
}
