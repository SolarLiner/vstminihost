use std::{
    path::Path,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
};

use tuix::*;

use nfd::Response;
use vst::{
    host::{Host, PluginInstance, PluginLoadError, PluginLoader},
    plugin::Plugin,
};

mod host;

#[derive(Default)]
struct VstHost;

impl Host for VstHost {}

#[derive(Clone, Debug, PartialEq)]
enum HostWidgetEvent {
    OpenFile,
}

#[derive(Default)]
pub struct HostWidget {
    label: Entity,
    host: Arc<Mutex<VstHost>>,
}

impl BuildHandler for HostWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        state.insert_stylesheet("src/theme.css");

        let container = VBox::new().build(state, entity, |builder| builder);

        let open_file_button = Button::with_label("Open File")
            .on_press(Event::new(HostWidgetEvent::OpenFile).target(entity))
            .build(state, container, |builder| {
                builder.class("open_file_dialogue_button")
            });

        self.label = Label::new("<Open a plugin first>").build(state, container, |builder| builder);

        entity
    }
}

impl EventHandler for HostWidget {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(host_widget_event) = event.message.downcast::<HostWidgetEvent>() {
            match host_widget_event {
                HostWidgetEvent::OpenFile => {
                    if event.target == entity {
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
                        let instance = load(self.host.clone(), path).map(|i| i.get_info());

                        self.label.set_text(state, &format!("{:?}", instance));

                        return true;
                    }
                }
            }
        }

        false
    }
}

fn load<H: Host, P: AsRef<Path>>(
    host: Arc<Mutex<H>>,
    path: P,
) -> Result<PluginInstance, PluginLoadError> {
    let mut loader = PluginLoader::load(path.as_ref(), Arc::clone(&host))?;
    loader.instance()
}

fn main() {
    let app = Application::new(move |win_desc, state, window| {
        let host_widget =
            HostWidget::default().build(state, window, |builder| builder.set_flex_grow(1.0));

        win_desc
    });

    app.run();
}
