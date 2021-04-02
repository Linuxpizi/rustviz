use rustviz_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, Owner, MutRef, StaticRef, Function, VisualizationData, Visualizable};
use rustviz_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;
fn main() {
    let x = ResourceAccessPoint::Owner(Owner {
        hash: 1,
        name: String::from("x"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::Copy,
    });
    let y = ResourceAccessPoint::Owner(Owner {
        hash: 2,
        name: String::from("y"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::Copy,
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
        preprocess_external_events: Vec::new(),
        event_line_map: BTreeMap::new()
    };

    vd.append_external_event(ExternalEvent::Duplicate{from: None,
        to: Some(x.clone())}, &(2 as usize));
    vd.append_external_event(ExternalEvent::Duplicate{from: None,
        to: Some(y.clone())}, &(3 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : x.clone() }, 
        &(5 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : y.clone() }, 
        &(5 as usize));

    //rendering image
    svg_generation::render_svg(&"examples/printing/input/".to_owned(), &"examples/printing/".to_owned(), &mut vd);
}