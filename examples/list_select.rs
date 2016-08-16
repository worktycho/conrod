#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};

fn main() {

    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 300;

    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("ListSelect Demo", [WIDTH, HEIGHT])
            .opengl(OpenGL::V3_2)
            .vsync(true)
            .samples(4)
            .exit_on_esc(true)
            .build()
            .unwrap();

    window.set_ups(60);

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // No text to draw, so we'll just create an empty text texture cache.
    let mut text_texture_cache =
        conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    // List of entries to display. They should implement the Display trait.
    let list_items = [
        "African Sideneck Turtle".to_string(),
        "Alligator Snapping Turtle".to_string(),
        "Common Snapping Turtle".to_string(),
        "Indian Peacock Softshelled Turtle".to_string(),
        "Eastern River Cooter".to_string(),
        "Eastern Snake Necked Turtle".to_string(),
        "Diamond Terrapin".to_string(),
        "Indian Peacock Softshelled Turtle".to_string(),
        "Musk Turtle".to_string(),
        "Reeves Turtle".to_string(),
        "Eastern Spiny Softshell Turtle".to_string(),
        "Red Ear Slider Turtle".to_string(),
        "Indian Tent Turtle".to_string(),
        "Mud Turtle".to_string(),
        "Painted Turtle".to_string(),
        "Spotted Turtle".to_string()
    ];

    // List of selections, should be same length as list of entries. Will be updated by the widget.
    let mut list_selected = vec![false; list_items.len()];

    // Make a default selection.
    list_selected[3] = true;

    // Poll events from the window.
    while let Some(event) = window.next() {

        // Convert the piston event to a conrod event.
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            use conrod::{widget, color, Colorable, Positionable, Sizeable, Widget};

            // Instantiate the conrod widgets.
            let ui = &mut ui.set_widgets();

            widget_ids!(CANVAS, LIST_SELECT);

            widget::Canvas::new().color(color::BLUE).set(CANVAS, ui);

            // Instantiate the `ListSelect` widget.
            let num_items = list_items.len();
            let item_h = 32.0;
            let (mut events, scrollbar) =
                widget::ListSelect::multiple(num_items, item_h)
                    .w_h(350.0, 220.0)
                    .top_left_with_margins_on(CANVAS, 40.0, 40.0)
                    .scrollbar_next_to()
                    .set(LIST_SELECT, ui);

            // Handle the `ListSelect`s events.
            while let Some(event) = events.next(ui, |i| list_selected[i]) {
                use widget::list_selected::Event;
                match event {

                    // For the `Item` events we instantiate the `List`'s items.
                    Event::Item(item) => {
                        let label = &list_items[item.i];
                        let (color, label_color) = match list_selected[item.i] {
                            true => (color::LIGHT_BLUE, color::YELLOW),
                            false => (color::LIGHT_GREY, color::BLACK),
                        };
                        let button = Button::new().color(color).label(label).label_color(label_color);
                        item.set(button, ui);
                    },

                    // The selection has changed.
                    Event::Selection(selection) => {
                        selection.update_bool_slice(&mut list_selected);
                        println!("selected indices: {:?}", &selection);
                    },

                    // The following events indicate interactions with the `ListSelect` widget.
                    Event::Press(_press) => (),
                    Event::Release(_release) => (),
                    Event::Click(_click) => (),
                    Event::DoubleClick(_double_click) => (),
                }
            }

            // Instantiate the scrollbar for the list.
            scrollbar.unwrap().set(ui);
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }
}
