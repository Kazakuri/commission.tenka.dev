use image::GenericImageView;

// Note: Adapted from https://github.com/image-rs/image/blob/88adf0cb3ba7e478439623949a1a515206e7a4ec/src/dynimage.rs#L807-L826
pub fn resize_to_fill(path: &str, nwidth: u32, nheight: u32) -> anyhow::Result<::image::DynamicImage> {
  let img = ::image::open(path)?;

  let (width2, height2) =
    resize_dimensions(img.width(), img.height(), nwidth, nheight, true);

  let mut intermediate = img.resize_exact(width2, height2, ::image::imageops::FilterType::Triangle);
  let (iwidth, iheight) = intermediate.dimensions();
  let ratio = u64::from(iwidth) * u64::from(nheight);
  let nratio = u64::from(nwidth) * u64::from(iheight);

  if nratio > ratio {
    Ok(intermediate.crop(0, 0, nwidth, nheight))
  } else {
    Ok(intermediate.crop((iwidth - nwidth) / 2, 0, nwidth, nheight))
  }
}

// Note: Taken from https://github.com/image-rs/image/blob/88adf0cb3ba7e478439623949a1a515206e7a4ec/src/math/utils.rs#L34-L66
fn resize_dimensions(width: u32, height: u32, nwidth: u32, nheight: u32, fill: bool) -> (u32, u32) {
  let ratio = u64::from(width) * u64::from(nheight);
  let nratio = u64::from(nwidth) * u64::from(height);

  let use_width = if fill {
      nratio > ratio
  } else {
      nratio <= ratio
  };
  let intermediate = if use_width {
      u64::from(height) * u64::from(nwidth) / u64::from(width)
  } else {
      u64::from(width) * u64::from(nheight) / u64::from(height)
  };
  let intermediate = std::cmp::max(1, intermediate);
  if use_width {
      if intermediate <= u64::from(::std::u32::MAX) {
          (nwidth, intermediate as u32)
      } else {
          (
              (u64::from(nwidth) * u64::from(::std::u32::MAX) / intermediate) as u32,
              ::std::u32::MAX,
          )
      }
  } else if intermediate <= u64::from(::std::u32::MAX) {
      (intermediate as u32, nheight)
  } else {
      (
          ::std::u32::MAX,
          (u64::from(nheight) * u64::from(::std::u32::MAX) / intermediate) as u32,
      )
  }
}