use itertools::Itertools;
use nannou::prelude::{Rect, Vector2};

pub struct WorldView {
    rect: Rect<f64>,
}

impl WorldView {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        let rect = Rect::from_x_y_w_h(x, y, width, height);

        Self { rect }
    }

    pub fn move_relative(&mut self, x: f64, y: f64) {
        self.rect = self.rect.shift(Vector2::new(x, y));
    }

    pub fn move_absolute(&mut self, x: f64, y: f64) {
        self.rect = Rect::from_x_y_w_h(x, y, self.rect.w(), self.rect.h());
    }

    pub fn xy(&self) -> Vector2<f64> {
        self.rect.xy()
    }

    pub fn grid_references_in_view(&self) -> Vec<(i32, i32)> {
        let (left, right, bottom, top) = self.rect.l_r_b_t();

        let x_steps = i32_steps(left, right, 1.0);
        let y_steps = i32_steps(bottom, top, 1.0);

        grid_refs_from_i32_steps(x_steps, y_steps)
    }
}

fn i32_steps(mut start: f64, end: f64, step_by: f64) -> Vec<i32> {
    let mut points = Vec::new();

    while start < end {
        points.push(start);
        start += step_by
    }

    points
        .into_iter()
        .map(f64::ceil)
        // .filter(|n| n > &end || n < &start)
        .map(|n| n as i32)
        .collect()
}

fn grid_refs_from_i32_steps(x_steps: Vec<i32>, y_steps: Vec<i32>) -> Vec<(i32, i32)> {
    x_steps
        .iter()
        .map(|x| y_steps.iter().map(move |y| (*x, *y)))
        .flatten()
        .unique()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_steps_x() {
        let expected_steps = vec![2, 3];
        let actual_steps = i32_steps(1.5, 3.5, 1.0);

        assert_eq!(expected_steps, actual_steps);
    }

    #[test]
    fn test_i32_steps_y() {
        let expected_steps = vec![0, 1];
        let actual_steps = i32_steps(-0.5, 1.5, 1.0);

        assert_eq!(expected_steps, actual_steps);
    }

    #[test]
    fn test_grid_refs_from_i32_steps_1x1() {
        let expected_grid_refs = vec![(5, 1)];
        let x_steps = i32_steps(4.5, 5.5, 1.0);
        let y_float_steps = i32_steps(0.5, 1.5, 1.0);
        let actual_grid_refs = grid_refs_from_i32_steps(x_steps, y_float_steps);

        assert_eq!(expected_grid_refs, actual_grid_refs);
    }

    #[test]
    fn test_grid_refs_from_i32_steps_2x2() {
        let expected_grid_refs = vec![(2, 0), (2, 1), (3, 0), (3, 1)];
        let x_steps = i32_steps(1.5, 3.5, 1.0);
        let y_float_steps = i32_steps(-0.5, 1.5, 1.0);
        let mut actual_grid_refs = grid_refs_from_i32_steps(x_steps, y_float_steps);
        actual_grid_refs.sort();

        assert_eq!(expected_grid_refs, actual_grid_refs);
    }

    #[test]
    fn test_grid_references_in_1x1_view() {
        let world_view = WorldView::new(5.0, 1.0, 1.0, 1.0);
        let expected_grid_references_in_view = vec![(5, 1)];
        let mut actual_grid_references_in_view = world_view.grid_references_in_view();
        // Ordering is not guaranteed so we always sort when checking against expected result
        actual_grid_references_in_view.sort();

        assert_eq!(
            expected_grid_references_in_view,
            actual_grid_references_in_view
        )
    }

    #[test]
    fn test_grid_references_in_2x2_view() {
        let world_view = WorldView::new(2.5, 0.5, 2.0, 2.0);
        let expected_grid_references_in_view = vec![(2, 0), (2, 1), (3, 0), (3, 1)];
        let mut actual_grid_references_in_view = world_view.grid_references_in_view();
        // Ordering is not guaranteed so we always sort when checking against expected result
        actual_grid_references_in_view.sort();

        assert_eq!(
            expected_grid_references_in_view,
            actual_grid_references_in_view
        )
    }

    #[test]
    fn test_grid_references_in_3x3_view() {
        let world_view = WorldView::new(7.0, 4.0, 3.0, 3.0);
        #[rustfmt::skip]
        let expected_grid_references_in_view = vec![
            (6, 3), (6, 4), (6, 5),
            (7, 3), (7, 4), (7, 5),
            (8, 3), (8, 4), (8, 5)
        ];
        let mut actual_grid_references_in_view = world_view.grid_references_in_view();
        // Ordering is not guaranteed so we always sort when checking against expected result
        actual_grid_references_in_view.sort();

        assert_eq!(
            expected_grid_references_in_view,
            actual_grid_references_in_view
        )
    }
}
