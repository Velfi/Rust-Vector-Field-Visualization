Spaces:
- `LineSegmentSpace`
  - Each line segment has it's own space wherein we can represent the position of the two points that make up a line segment
- `WorldSpace`
  - A shared space wherein we can represent the position of all `LineSegment`s relative to one another.
- `ViewSpace`
  - A space encompassing `WorldSpace` allowing us to transform it relative to a camera's viewpoint.
- `ProjectionSpace`
  - A space allowing for the translation of `ViewSpace` coordinates into screen space coordinates (ranging from `-1.0` to `1.0`) so that 3D objects can be displayed on a 2D screen.
