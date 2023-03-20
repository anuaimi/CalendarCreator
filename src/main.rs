extern crate printpdf;

use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    // doc is PdfDocumentReference, page1 is a page index, layer1 a layer index
    let (doc_ref, page1, layer1) = PdfDocument::new("Calendar", Mm(203.0), Mm(260.0), "Layer 1");

    let mut doc = doc_ref;
    doc = doc.with_title("replacement title");
    doc = doc.with_author("athir nuaimi");
    // doc = doc.with_document_version(1);

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // let fill_color = Color::Cmyk(Cmyk::new(0.0, 0.23, 0.0, 0.0, None));
    // current_layer.set_fill_color(fill_color);
  
    let points1 = vec![
        (Point::new(Mm(50.0), Mm(100.0)), false),
        (Point::new(Mm(50.0), Mm(200.0)), false),
        (Point::new(Mm(150.0), Mm(200.0)), false),
        (Point::new(Mm(150.0), Mm(100.0)), false),
    ];
    let line1 = Line {
        points: points1,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };
    current_layer.add_shape(line1);

    let radius = Pt(40.0);
    let offset_x = Pt(10.0);
    let offset_y = Pt(50.0);

    let line = Line {
        points: calculate_points_for_circle(radius, offset_x, offset_y),
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };
    current_layer.add_shape(line);

    doc.save(&mut BufWriter::new(File::create("calendar.pdf").unwrap()))
        .unwrap();
}
