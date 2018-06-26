extern crate radiotap;

fn main() {
    let frame = [
        0, 0, 39, 0, 46, 72, 0, 192, 0, 0, 0, 128, 0, 0, 0, 160, 4, 0, 0, 0, 16, 2, 158, 9, 160, 0,
        227, 5, 0, 0, 255, 255, 255, 255, 2, 0, 222, 173, 4,
    ];

    // let radiotap = radiotap::Radiotap::from_bytes(&frame).unwrap() // gives you a parsed Radiotap object

    // let (radiotap, rest) = radiotap::Radiotap::parse(&frame).unwrap() // gives you a parsed Radiotap object and the rest slice

    // let radiotap = Radiotap::new();

    // let capture = radiotap::CaptureNamespace::new().default(&mut radiotap).from_bytes(&frame).unwrap() // returns a Capture

    // let(capture, rest) = radiotap::CaptureParser::new().default(&mut radiotap).parse(&frame).unwrap() // returns a Capture and the rest slice

    // radiotap::CaptureIterator




    // radiotap::Capture::new()

    // for element in radiotap::CaptureIterator::new()
    //     .default(&radiotap)
    //     .vendor(&vns)
    //     .from_bytes(&frame)
    // {
    //     match element {
    //         Ok((oui, ))
    //     }
    // }

    // let apple = Apple::new();
    // let (capture, rest) = radiotap::Capture::new().vendor(&apple).parse(&frame);

    // let (radiotap, rest) = radiotap::Radiotap::parse(&frame);

    // let radiotap = radiotap::Radiotap::from_bytes(&frame);

    // if let Err(e) = radiotap::CaptureIterator::from_bytes(&frame) {
    //     eprintln!("Fatal error: {}", e);
    //     for e in e.causes().skip(1) {
    //         eprintln!("     Due to: {}", e);
    //     }
    // }
}
