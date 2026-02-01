use std::time::Duration;

use smithay::{
    backend::{
        renderer::{
            damage::OutputDamageTracker, element::surface::WaylandSurfaceRenderElement, gles::GlesRenderer,
        },
        winit::{init, WinitEvent, WinitEventLoop, WinitGraphicsBackend},
    },
    output::{Mode, Output},
    reexports::{
        calloop::{EventLoop, InsertError},
        wayland_protocols::ext::background_effect::v1::server::ext_background_effect_surface_v1::Error,
    },
    utils::{Rectangle, Transform},
};

use crate::{CalloopData, MiniWm, backends::{Backend, winit}};

/* This is the struct for the winit backend, we init winit, define the output and the mode of the
 * screen and setup the damage tracker so we know when to re render the client.
 * We also setup the event sources for the calloop event loop by registrying the Winit events.
*/
pub struct WinitBackend {
    output: Output,
    backend: WinitGraphicsBackend<GlesRenderer>,
    damage_tracker: OutputDamageTracker,
}

impl WinitBackend {
    pub fn new(state: &mut MiniWm) -> Result<(Self, WinitEventLoop), Box<dyn std::error::Error>> {
        let display_handle = &mut state.display_handle;

        let (backend, winit_event_loop) = winit::init()?;

        let mode = Mode {
            size: backend.window_size(),
            refresh: 60_000,
        };

        let output = Output::new(
            "winit".to_string(),
            smithay::output::PhysicalProperties {
                size: (0, 0).into(),
                subpixel: smithay::output::Subpixel::Unknown,
                make: "Smithay".into(),
                model: "Winit".into(),
            },
        );

        let _global = output.create_global::<MiniWm>(display_handle);
        output.change_current_state(Some(mode), Some(Transform::Flipped180), None, Some((0, 0).into()));
        output.set_preferred(mode);

        state.space.map_output(&output, (0, 0));

        let damage_tracker = OutputDamageTracker::from_output(&output);

        std::env::set_var("WAYLAND_DISPLAY", &state.socket_name);

        Ok((
            Self {
                output,
                backend,
                damage_tracker,
            },
            winit_event_loop,
        ))
    }

    pub fn init(
        &mut self,
        winit_event_loop: WinitEventLoop,
        event_loop: &mut EventLoop<CalloopData>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        event_loop
            .handle()
            .insert_source(winit_event_loop, move |event, _, data| {
                let display = &mut data.display_handle;
                let state = &mut data.state;
                let winit_backend = match &mut data.backend {
                    Backend::Winit(winit_backend) => winit_backend,
                };

                match event {
                    WinitEvent::Resized { size, .. } => {
                        winit_backend.output.change_current_state(
                            Some(Mode {
                                size,
                                refresh: 60_000,
                            }),
                            None,
                            None,
                            None,
                        );
                    }
                    // how we handle input events
                    WinitEvent::Input(event) => state.process_input_event(event),
                    WinitEvent::Redraw => {
                        let size = winit_backend.backend.window_size();
                        let damage = Rectangle::from_size(size);

                        {
                            let (renderer, mut framebuffer) = winit_backend.backend.bind().unwrap();
                            smithay::desktop::space::render_output::<
                                _,
                                WaylandSurfaceRenderElement<GlesRenderer>,
                                _,
                                _,
                            >(
                                &winit_backend.output,
                                renderer,
                                &mut framebuffer,
                                1.0,
                                0,
                                [&state.space],
                                &[],
                                &mut winit_backend.damage_tracker,
                                [0.1, 0.1, 0.1, 1.0],
                            )
                            .unwrap();
                        }
                        winit_backend.backend.submit(Some(&[damage])).unwrap();

                        state.space.elements().for_each(|window| {
                            window.send_frame(
                                &winit_backend.output,
                                state.start_time.elapsed(),
                                Some(Duration::ZERO),
                                |_, _| Some(winit_backend.output.clone()),
                            )
                        });

                        state.space.refresh();
                        state.popups.cleanup();
                        let _ = display.flush_clients();

                        // Ask for redraw to schedule new frame.
                        winit_backend.backend.window().request_redraw();
                    }
                    WinitEvent::CloseRequested => {
                        state.loop_signal.stop();
                    }
                    _ => (),
                };
            })?;

        Ok(())
    }
}
