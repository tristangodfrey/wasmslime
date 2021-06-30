use model::cell::{CellMap};
use model::trail_map::*;
use model::config::SimulationConfig;
use model::simulation::Simulation;
use model::point::*;
use yew::prelude::*;
use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsCast;
use web_sys::ImageData;
use wasm_bindgen::Clamped;
use yew::services::console::ConsoleService as Console;

mod model;

enum Msg {
    Step,
    TrailCanvasClick(MouseEvent)
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    cell_canvas: NodeRef,
    trail_canvas: NodeRef,
    composite_canvas: NodeRef,
    simulation: Simulation,
    live_cell_count: usize
}

pub fn rand() -> f64 {
    let mut val: [u8; 1] = [0];

    match getrandom::getrandom(&mut val) {
        Ok(()) => {

        },
        Err(err) => {
            Console::log(&format!("Failed to generate random number! {:?}", err));
        }
    }

    val[0] as f64 / 255f64

}

fn get_context(canvas: HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let mut config = SimulationConfig::default();

        config.width = 200;
        config.height = 200;

        let cell_map = CellMap::new_random(config.width, config.height, config.sensor_config.clone(), rand, 0.1f64);
        let trail_map = TrailMap::new_random(config.width, config.height, rand);

        Self {
            link,
            value: 0,
            cell_canvas: NodeRef::default(),
            trail_canvas: NodeRef::default(),
            composite_canvas: NodeRef::default(),
            simulation: Simulation::new(config, cell_map, trail_map, rand),
            live_cell_count: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Step => {
                //do the step
                self.simulation.step();
                
                //render shit
                if let Some(cell_canvas) = self.cell_canvas.cast::<HtmlCanvasElement>() {
                    let context = get_context(cell_canvas);

                    let image_data: ImageData = self.simulation.cell_map.clone().into();

                    context.put_image_data(&image_data, 0f64, 0f64);
                    
                }

                if let Some(trail_canvas) = self.trail_canvas.cast::<HtmlCanvasElement>() {
                    let context = get_context(trail_canvas);

                    let image_data: ImageData = self.simulation.trail_map.clone().into();

                    context.put_image_data(&image_data, 0f64, 0f64);
                }

                if let Some(composite_canvas) = self.composite_canvas.cast::<HtmlCanvasElement>() {
                    //@TODO: implement this
                }

                self.live_cell_count = self.simulation.cell_map.live_cells();

                true
            },
            Msg::TrailCanvasClick(event) => {
                let (x, y) = (event.offset_x(), event.offset_y());

                Console::log(&format!("Got a mouse click @ {:?},{:?}", x, y));

                

                true
            }
        }
    }

    fn rendered(&mut self, _: bool)
    {
        if let Some(canvas) = self.cell_canvas.cast::<HtmlCanvasElement>() {

            let context = get_context(canvas);


            let pixel = [255, 110, 100, 255];

            let mut data = Vec::new();

            let clamped: Clamped<&[u8]> = Clamped(&data);

            

            
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {

        let trail_canvas_click = self.link.callback(|e: MouseEvent| Msg::TrailCanvasClick(e));

        html! {
            <div>
                <p>{ format!("Live cell count: {}", self.live_cell_count) }</p>
                <button onclick=self.link.callback(|_| Msg::Step)>{ "Step" }</button>
                <canvas ref=self.cell_canvas.clone() width=self.simulation.config.width.to_string() height=self.simulation.config.height.to_string()></canvas>
                <canvas
                    ref=self.trail_canvas.clone()
                    width=self.simulation.config.width.to_string()
                    onclick=trail_canvas_click
                    height=self.simulation.config.height.to_string()>
                </canvas>
                <canvas ref=self.composite_canvas.clone() width=self.simulation.config.width.to_string() height=self.simulation.config.height.to_string()></canvas>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}