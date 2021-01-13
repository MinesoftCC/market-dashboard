use bytes::Bytes;

static DEFAULT_ICON: &[u8] = b"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";

#[derive(Default, Clone, Debug)]
pub struct Image {
    pub id: Option<egui::TextureId>,
    pub size: (usize, usize),
    pub pixels: Vec<egui::Color32>,
}

impl Image {
    pub fn as_texture(&mut self, frame: &mut epi::Frame<'_>) -> Option<egui::TextureId> {
        if self.id.is_none() && self.size != (0, 0) && !self.pixels.is_empty() {
            let texture_allocator = frame.tex_allocator().as_mut().unwrap();
            self.id = Some(texture_allocator.alloc());

            texture_allocator.set_srgba_premultiplied(self.id.unwrap(), self.size, &self.pixels);
        }

        self.id
    }

    pub fn from_url(url: &str) -> Self {
        use image::GenericImageView;

        let client = reqwest::blocking::Client::new();
        let response = match client.get(url).send() {
            Ok(response) => response.bytes().unwrap(),
            Err(e) => {
                eprintln!("An error occured for URL \"{}\":\n{:#?}", url, e);

                println!("Falling back to default icon...");
                Bytes::from(base64::decode(DEFAULT_ICON).unwrap())
            },
        };

        let response_bytes = response.to_vec();

        let image = if let Ok(i) = image::load_from_memory(&response_bytes) {
            i
        } else {
            eprintln!("Could not build image from response bytes");
            return Image {
                id: None,
                size: (0, 0),
                pixels: vec![],
            };
        };

        let image_buffer = image.to_rgba8();
        let size = (image.width() as usize, image.height() as usize);
        let pixels = image_buffer.into_vec();

        assert_eq!(size.0 * size.1 * 4, pixels.len());

        let image = Self {
            id: None,
            size,
            pixels: pixels
                .chunks(4)
                .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                .collect(),
        };

        image
    }
}
