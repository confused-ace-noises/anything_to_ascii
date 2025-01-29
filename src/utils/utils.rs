pub trait DemureUnwrap<T> {
    fn demure_unwrap(&self, src_width: T, src_height: T) -> (T, T);
}

impl DemureUnwrap<usize> for (Option<usize>, Option<usize>) {
    fn demure_unwrap(&self, src_width: usize, src_height: usize) -> (usize, usize) {
        match self {
            (None, None) => return (src_width, src_height/2),
            (None, Some(height)) => {
                let ratio = src_height as f32 / src_width as f32;
                let height = *height;

                (
                    (((height as f32 / ratio) * 2.0).ceil() as usize).clamp(1, src_height),
                    height,
                )
            }
            (Some(width), None) => {
                let ratio = src_height as f32 / src_width as f32;
                let width = *width;

                (
                    width,
                    (((width as f32 * ratio) / 2.0).ceil() as usize).clamp(1, src_height),
                )
            }
            (Some(width), Some(height)) => (*width, *height),
        }
    }
}
