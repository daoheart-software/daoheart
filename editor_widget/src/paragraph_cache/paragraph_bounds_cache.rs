/// Keeps information on the top pixel position of each paragraph. Whenever paragraph with
/// index n resizes, truncates the tops to n because all the paragraphs below will have moved
/// up or down by the delta height of paragraph n. Calculating the heights is done by iterating
/// over the heights from n + 1 above and adding the height of the current paragraph to the
/// bottom of the previous one.
///
/// This structure allows to get the paragraphs that should be visible within some pixel range
/// (the viewport).
#[derive(Debug)]
pub(crate) struct ParagraphBoundsCache {
    pub(super) heights: Vec<f32>,
    pub(super) tops: Vec<f32>,
}

pub(crate) struct ParagraphBounds {
    pub top: f32,
    pub height: f32,
}

impl ParagraphBounds {
    pub fn bottom(&self) -> f32 {
        self.top + self.height
    }
}

impl ParagraphBoundsCache {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heights: Vec::with_capacity(capacity),
            tops: Vec::with_capacity(capacity),
        }
    }

    pub fn lines_len(&self) -> usize {
        self.heights.len()
    }

    pub fn calc_bounds(&mut self, paragraph_idx: usize) {
        if self.tops.len() <= paragraph_idx {
            let mut current_top = if self.tops.is_empty() {
                0.0
            } else {
                // Start from the last known valid top + the last known height
                let last_idx = self.tops.len() - 1;
                self.tops[last_idx] + self.heights[last_idx]
            };

            for i in self.tops.len()..=paragraph_idx {
                self.tops.push(current_top);
                current_top += self.heights[i];
            }
        }
    }

    pub fn get_bounds(&self, idx: usize) -> Option<ParagraphBounds> {
        if self.tops.len() <= idx {
            None
        } else {
            Some(ParagraphBounds {
                top: self.tops[idx],
                height: self.heights[idx],
            })
        }
    }

    pub fn set_height(&mut self, idx: usize, new_height: f32) {
        if self.heights.len() == idx {
            self.heights.push(new_height);
            let new_top = self.tops.last().unwrap_or(&0.0) + new_height;
            self.tops.push(new_top);
            return;
        } else if self.heights.len() < idx {
            panic!("Tried to set line height on index greater than max index + 1!");
        }

        if self.heights[idx] == new_height && self.tops.len() > idx {
            return;
        }

        self.heights[idx] = new_height;

        if self.tops.len() > idx {
            self.tops.truncate(idx);
        }

        let prev_top = if idx == 0 { 0.0 } else { self.tops[idx - 1] };
        self.tops.push(prev_top + new_height);
    }

    pub fn find_nearest_paragraph(&self, top_px: f32) -> usize {
        self.tops
            .binary_search_by(|probe| probe.total_cmp(&top_px))
            .unwrap_or_else(|insertion_index| insertion_index.saturating_sub(1))
    }
}
