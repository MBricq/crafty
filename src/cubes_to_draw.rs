use crate::cube::Cube;
use crate::graphics::cube::CubeInstance;
use crate::primitives::vector::Vector3;
use crate::chunk::Chunk;

/// Control the cubes to be drawn
pub struct CubesToDraw {
    /// List of cube that are going to be drawn
    // Idea for futur: split the vec into smaller structure "ChunkToDraw" representing a "physical" chunk
    // it will be easier to load/unload a "physical" chunk
    // apparently glium can build its buffer vector from multiple references 
    // so no need to copy everythink into one vector but it has to be tested (I haven't tested it)
    cubes_to_draw: Vec<CubeInstance>,
    selected_cube_index: Option<usize>,
}

impl CubesToDraw {
    pub fn new() -> Self {
        Self { cubes_to_draw: Vec::new(), selected_cube_index: None }
    }

    /// Set the vector of CubeAttr from parameter
    pub fn set_cube_to_draw(&mut self, cubes_to_draw: Vec<CubeInstance>) {
        self.cubes_to_draw = cubes_to_draw;
    }

    /// Add a CubeAttr to the Vector from the parameter of a Cube
    pub fn add_cube(&mut self, c: &Cube) {
        self.cubes_to_draw.push(CubeInstance::new(c));
    }

    /// Try to remove a cube at at position, 
    /// Will not panic if a cubeAttr is not present in the Vec
    pub fn remove_cube(&mut self, position: &Vector3) {
        for i in 0..self.cubes_to_draw.len() {
            if self.cubes_to_draw[i].position() == position.as_array() {
                // Potentially unselect the cube if it was
                if self.selected_cube_index == Some(i) {
                    self.selected_cube_index = None;
                }
                self.cubes_to_draw.swap_remove(i);
                return;
            }
        }
    }

    pub fn set_selected_cube(&mut self, selected_cube: Option<Vector3>) {
        return;
        // Unselect last cube
        if let Some(index) = self.selected_cube_index {
            self.cubes_to_draw[index].set_is_selected(false);
        }
        if selected_cube.is_none() { return; }
        // Select new cube
        // TODO could we just add a cube over the others that for the selected one ?
        for (i, cube) in self.cubes_to_draw.iter_mut().enumerate() {
            if cube.position() == selected_cube.unwrap().to_cube_coordinates().as_array() {
                self.selected_cube_index = Some(i);
                cube.set_is_selected(true);
                return;
            }
        }
    }

    pub fn cubes_to_draw(&self) -> &Vec<CubeInstance> {
        &self.cubes_to_draw
    }

    pub fn number_cubes_rendered(&self) -> usize {
        self.cubes_to_draw.len()
    }

    /// Add the corresponding CubeAttr from he cube in a chunk
    pub fn add_chunk(&mut self, chunk: &Chunk) {
        for layer in chunk.cubes() {
            for row in layer {
                for cube in row {
                    if let Some(c) = cube {
                        self.add_cube(c)
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::cube::Cube;

    use super::CubesToDraw;
    use crate::block_kind::Block::DIRT;
    use crate::primitives::vector::Vector3;

    #[test]
    fn test_add_remove_one_cube() {
        let mut cube_to_draw = CubesToDraw::new();
        let mut vec_to_draw = Vec::new();
        let world_matrix = [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]];
        vec_to_draw.push(Cube::new([0., 0., 0.], DIRT, 0));

        cube_to_draw.add_cube(&Cube::new([0., 0., 0.], DIRT, 0));

        assert!(cube_to_draw.cubes_to_draw().len() == 1);

        cube_to_draw.remove_cube(&Vector3::newf([1., 0., 0.]));
        assert!(cube_to_draw.cubes_to_draw().len() == 1);

        cube_to_draw.remove_cube(&Vector3::newf([0., 0., 0.]));
        assert!(cube_to_draw.cubes_to_draw().len() == 0);
    }
}