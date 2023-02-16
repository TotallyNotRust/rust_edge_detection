
use opencv::core;
use opencv::core::no_array;
use opencv::core::BORDER_DEFAULT;
use opencv::core::CV_CN_SHIFT;
use opencv::gapi::y;
use opencv::highgui;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::imgproc::CHAIN_APPROX_SIMPLE;
use opencv::imgproc::CHAIN_APPROX_TC89_KCOS;
use opencv::imgproc::COLOR_BGR2GRAY;
use opencv::imgproc::LINE_8;
use opencv::imgproc::LINE_AA;
use opencv::imgproc::MORPH_CLOSE;
use opencv::imgproc::MORPH_RECT;
use opencv::imgproc::RETR_EXTERNAL;
use opencv::imgproc::RETR_TREE;
use opencv::imgproc::THRESH_BINARY;
use opencv::imgproc::THRESH_BINARY_INV;
use opencv::types;
use opencv::Error;


pub fn scan(file_path: String) -> Result<Vec<String>, Error> {
    let mut image = imgcodecs::imread(&file_path, -1).unwrap();

    let mut grey_image = core::Mat::default();

    imgproc::cvt_color(&image, &mut grey_image, COLOR_BGR2GRAY, 0)?;

    let mut median_blurred_image = core::Mat::default();

    imgproc::median_blur(&grey_image, &mut median_blurred_image, 5)?;

    let mut filter_2d_image = core::Mat::default();

    let filter = core::Mat::from_slice_2d(&[[-1, -1, -1], [-1, 9, -1], [-1, -1, -1]])?;

    imgproc::filter_2d(
        &median_blurred_image,
        &mut filter_2d_image,
        -1,
        &filter,
        core::Point::new(-1, -1),
        0.0,
        BORDER_DEFAULT,
    )?;

    let mut thresheld_image = core::Mat::default();

    let threshold = imgproc::threshold(
        &filter_2d_image,
        &mut thresheld_image,
        160.0,
        255.0,
        THRESH_BINARY,
    )?;

    println!("threshold {:?}", &threshold);

    let kernel = imgproc::get_structuring_element(
        MORPH_RECT,
        core::Size::new(3, 3),
        core::Point::new(0, 0),
    )?;

    println!("Kernel: {:?}", &kernel);

    let mut close_image = core::Mat::default();

    imgproc::morphology_ex(
        &thresheld_image,
        &mut close_image,
        MORPH_CLOSE,
        &kernel,
        core::Point::new(-1, -1),
        1,
        BORDER_DEFAULT,
        core::Scalar::default(),
    )?;

    let mut cnts = types::VectorOfMat::new();
    ();

    imgproc::find_contours(
        &close_image,
        &mut cnts,
        RETR_TREE,
        CHAIN_APPROX_TC89_KCOS,
        core::Point::new(0, 0),
    )?;

    let idx: i32 = 0;
    let thickness: i32 = 4;
    const WHITE_COLOR: f64 = 255 as f64;
    let color = core::Scalar::new(WHITE_COLOR, WHITE_COLOR, WHITE_COLOR, WHITE_COLOR);
    let zero_offset = core::Point::new(0, 0);
    let maxresult: i32 = 1000;
    let hierachy = types::VectorOfMat::new();
    //~ let empty_mat = core::Mat::default().unwrap();
    //~ hierachy.push(empty_mat);

    let mut ctr_image = image.clone();

    imgproc::draw_contours(
        &mut ctr_image,
        &cnts,
        idx,
        color,
        thickness,
        LINE_AA,
        &no_array(),
        maxresult,
        zero_offset,
    )?;

    let files: Vec<String> = vec![];

    let mut i = 0;
    for x in cnts {
        let area = imgproc::contour_area(&x, false)?;
        // let rect = imgproc::bounding_rect(&x)?;
        // println!("{:?}", area);
        
        if area > 10000.0 {
            let rect = imgproc::bounding_rect(&x)?;
            println!("Found rect {:?}", rect);
            imgproc::rectangle(&mut ctr_image, rect, color, thickness, LINE_8, 0)?;

            let temp_img = core::Mat::roi(&mut image, rect)?;

            imgcodecs::imwrite(&format!("{}{}.jpg", file_path, i), &temp_img, &vec![].into())?;

            files.push(format!("{}{}.jpg", file_path, i));

            i+=1;
        }
    }
    Ok(files)
}
