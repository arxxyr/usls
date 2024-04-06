use usls::{models::RTDETR, Annotator, DataLoader, Options, COCO_NAMES_80};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build model
    let options = Options::default()
        .with_model("../models/rtdetr-l-f16.onnx")
        .with_confs(&[0.4, 0.15]) // person: 0.4, others: 0.15
        .with_names(&COCO_NAMES_80);
    let mut model = RTDETR::new(&options)?;

    // load image
    let x = vec![DataLoader::try_read("./assets/bus.jpg")?];

    // run
    let y = model.run(&x)?;

    // annotate
    let annotator = Annotator::default().with_saveout("RT-DETR");
    annotator.annotate(&x, &y);

    Ok(())
}