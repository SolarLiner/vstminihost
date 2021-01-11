use host::VstHost;
use tuix::*;

use nfd::Response;

mod host;

#[derive(Clone, Debug, PartialEq)]
enum HostWidgetEvent {
    OpenFile,
}

#[derive(Default)]
pub struct HostWidget {
    host: VstHost,
}

impl HostWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BuildHandler for HostWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        state.insert_stylesheet("src/theme.css");

        let open_file_button = Button::with_label("Open File")
            .on_press(Event::new(HostWidgetEvent::OpenFile).target(entity))
            .build(state, entity, |builder| {
                builder.class("open_file_dialogue_button")
            });

        entity
    }
}

impl EventHandler for HostWidget {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(host_widget_event) = event.message.downcast::<HostWidgetEvent>() {
            match host_widget_event {
                HostWidgetEvent::OpenFile => {
                    let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
                        panic!(e);
                    });

                    let path = match result {
                        Response::Okay(file_path) => file_path,
                        Response::OkayMultiple(mut files) => {
                            files.drain(..).take(1).next().unwrap()
                        }
                        _ => return false,
                    };
                    match self.host.load(path) {
                        Ok(()) => {}
                        Err(err) => eprintln!("Plugin load error: {}", err),
                    }
                }
            }
        }

        false
    }
}

fn main() {
    let app = Application::new(|win_desc, state, window| {
        let host_widget =
            HostWidget::new().build(state, window, |builder| builder.set_flex_grow(1.0));

        win_desc
    });

    app.run();
}
