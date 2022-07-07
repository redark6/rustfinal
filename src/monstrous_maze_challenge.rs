use challenge_trait::ChallengeTrait;
pub (crate) mod challenge_trait;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonstrousMazeOutput {
    pub path: String
}

pub struct MonstrousMaze {
    pub input: MonstrousMazeInput,
    pub maze: Vec<String>,
    pub startPoint: (u64, u64),
    pub endPoint: (u64, u64),
}

struct Grid {
    grid: Vec<String>,
    start: (u64, u64),
    end: (u64, u64),
}

struct GridPossibleSolution {
    current_coordinates: (i64, i64),
    path_taken: String,
    visited_coordinates: Vec<(i64, i64)>,
    encoutered_monster: i64,
    success: bool,
}

impl MonstrousMaze {
    const START_CHARACTER: char = 'Y';
    const END_CHARACTER: char = 'X';
    const MONSTER_CHARACTER: char = 'M';
    const FREE_WAY_CHARACTER: char = ' ';

    fn stringified_maze_to(mazeString: String) -> (Vec<String>, (u64, u64), (u64, u64)) {
        let maze: Vec<String> = mazeString.lines().map(str::to_string).collect();
        let mut startPoint: (u64, u64) = (0, 0);
        let mut endPoint: (u64, u64) = (0, 0);

        for (y, row) in maze.iter().enumerate() {
            let startX = row.find(MonstrousMaze::START_CHARACTER) ;
            let endX = row.find(MonstrousMaze::END_CHARACTER) ;
            if startX.is_some() { startPoint = ( y as u64, startX.unwrap() as u64 -2)}
            if endX.is_some() { endPoint = (y as u64, endX.unwrap() as u64 -2 )}
        }

        (maze, startPoint, endPoint)
    }

    fn is_coordinates_in_grid( coordinate: (i64,i64), grid: &Grid) -> bool{
        if coordinate.0 > -1 && coordinate.1 > -1 && coordinate.0 < grid.grid.len() as i64 && coordinate.1 < grid.grid[coordinate.0 as usize].len() as i64{
            let current_line: String = grid.grid[coordinate.0 as usize].clone();
            let current_char: char = current_line.chars().nth(coordinate.1 as usize).unwrap() as char;
            if current_char == MonstrousMaze::START_CHARACTER || current_char == MonstrousMaze::END_CHARACTER || current_char == MonstrousMaze::MONSTER_CHARACTER || current_char == MonstrousMaze::FREE_WAY_CHARACTER {
                return true
            }
            return false;
        }
        return false
    }

    fn is_coordinates_monster( coordinate: (i64,i64), grid: &Grid) -> bool{
        let current_line: String = grid.grid[coordinate.0 as usize].clone();
        let current_char: char = current_line.chars().nth(coordinate.1 as usize).unwrap() as char;
        return current_char == MonstrousMaze::MONSTER_CHARACTER;
    }

    fn find_paths(grid: &Grid, mut grid_possible_solution: GridPossibleSolution) -> Vec<GridPossibleSolution> {
        if grid_possible_solution.visited_coordinates.contains(&grid_possible_solution.current_coordinates) {
            return vec![];
        }
        grid_possible_solution.visited_coordinates.push(grid_possible_solution.current_coordinates);

        let mut paths: Vec<GridPossibleSolution> = vec![];

        //println!("Current coordinates : {:?}", grid_possible_solution.current_coordinates);
        //println!("Current path taken : {:?}", grid_possible_solution.path_taken);
        let current_line: String = grid.grid[grid_possible_solution.current_coordinates.0 as usize].clone();
        let current_char: char = current_line.chars().nth(grid_possible_solution.current_coordinates.1 as usize).unwrap() as char;
        //println!("Current char: {}", current_char);

        return if current_char == MonstrousMaze::START_CHARACTER ||
                  current_char == MonstrousMaze::END_CHARACTER ||
                  current_char == MonstrousMaze::MONSTER_CHARACTER ||
                  current_char == MonstrousMaze::FREE_WAY_CHARACTER {
            if current_char == MonstrousMaze::END_CHARACTER {
                grid_possible_solution.success = true;
                paths.push(grid_possible_solution);
                return paths;
            }

            let mut all_paths: Vec<GridPossibleSolution> = vec![];

            let right_direction = '>';
            let right_coordinates = (grid_possible_solution.current_coordinates.0, grid_possible_solution.current_coordinates.1 + 1);
            if MonstrousMaze::is_coordinates_in_grid(right_coordinates, grid) {
                let mut monster = 0;
                if MonstrousMaze::is_coordinates_monster(right_coordinates, grid) {
                    monster += 1;
                }
                //println!("Going : right");
                let mut visited_coordinates = grid_possible_solution.visited_coordinates.clone();
                let right_grid_possible_solution = GridPossibleSolution {
                    current_coordinates: right_coordinates,
                    path_taken: format!("{}{}", grid_possible_solution.path_taken.clone(), right_direction),
                    visited_coordinates,
                    encoutered_monster: grid_possible_solution.encoutered_monster + monster,
                    success: false,
                };
                all_paths.append(&mut MonstrousMaze::find_paths(&grid, right_grid_possible_solution));
            }

            let top_direction = '^';
            let top_coordinates = (grid_possible_solution.current_coordinates.0 - 1, grid_possible_solution.current_coordinates.1);
            if MonstrousMaze::is_coordinates_in_grid(top_coordinates, grid) {
                let mut monster = 0;
                if MonstrousMaze::is_coordinates_monster(top_coordinates, grid) {
                    monster += 1;
                }
                //println!("Going : top");
                let mut visited_coordinates = grid_possible_solution.visited_coordinates.clone();
                let top_grid_possible_solution = GridPossibleSolution {
                    current_coordinates: top_coordinates,
                    path_taken: format!("{}{}", grid_possible_solution.path_taken.clone(), top_direction),
                    visited_coordinates,
                    encoutered_monster: grid_possible_solution.encoutered_monster + monster,
                    success: false,
                };
                all_paths.append(&mut MonstrousMaze::find_paths(&grid, top_grid_possible_solution));
            }

            let left_direction = '<';
            let left_coordinates = (grid_possible_solution.current_coordinates.0, grid_possible_solution.current_coordinates.1 - 1);
            if MonstrousMaze::is_coordinates_in_grid(left_coordinates, grid) {
                let mut monster = 0;
                if MonstrousMaze::is_coordinates_monster(left_coordinates, grid) {
                    monster += 1;
                }
                //println!("Going : left");
                let mut visited_coordinates = grid_possible_solution.visited_coordinates.clone();
                let left_grid_possible_solution = GridPossibleSolution {
                    current_coordinates: left_coordinates,
                    path_taken: format!("{}{}", grid_possible_solution.path_taken.clone(), left_direction),
                    visited_coordinates,
                    encoutered_monster: grid_possible_solution.encoutered_monster + monster,
                    success: false,
                };
                all_paths.append(&mut MonstrousMaze::find_paths(&grid, left_grid_possible_solution));
            }


            let bottom_direction = 'v';
            let bottom_coordinates = (grid_possible_solution.current_coordinates.0 + 1, grid_possible_solution.current_coordinates.1);
            if MonstrousMaze::is_coordinates_in_grid(bottom_coordinates, grid) {
                let mut monster = 0;
                if MonstrousMaze::is_coordinates_monster(bottom_coordinates, grid) {
                    monster += 1;
                }
                //println!("Going : bottom");
                let mut visited_coordinates = grid_possible_solution.visited_coordinates.clone();
                let bottom_grid_possible_solution = GridPossibleSolution {
                    current_coordinates: bottom_coordinates,
                    path_taken: format!("{}{}", grid_possible_solution.path_taken.clone(), bottom_direction),
                    visited_coordinates,
                    encoutered_monster: grid_possible_solution.encoutered_monster + monster,
                    success: false,
                };
                all_paths.append(&mut MonstrousMaze::find_paths(&grid, bottom_grid_possible_solution));
            }

            //println!("\n======\n");
            all_paths
        } else {
            paths
        }
    }

}

impl ChallengeTrait for MonstrousMaze {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        "monstrousMaze".to_string()
    }

    fn new(input: Self::Input) -> Self {
        let (maze, startPoint, endPoint) = MonstrousMaze::stringified_maze_to("│Y M X│".to_string());
        return MonstrousMaze { input, maze, startPoint, endPoint };
    }

    fn solve(&self) -> Self::Output {
        let start = self.startPoint.clone();
        let results = MonstrousMaze::find_paths(
            &Grid {
            grid: self.maze.clone(),
            start: self.startPoint.clone(),
            end: self.endPoint.clone()
        },
            GridPossibleSolution {
            current_coordinates: ( start.0 as i64, start.1 as i64) ,
            path_taken: "".to_string(),
            visited_coordinates: vec![],
            encoutered_monster: 0,
            success: false
        } );
        for result in results {
            let res = MonstrousMazeOutput{path: result.path_taken.clone()};
            if self.verify(&res)  && result.encoutered_monster < self.input.endurance.into() { return res; }
        }
        MonstrousMazeOutput{path: "".to_string()}
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let chars: Vec<char> = answer.path.chars().collect();
        let mut startX = self.startPoint.1.clone();
        let mut startY = self.startPoint.0.clone();
        for c in chars {
            if c == '>' { startX+=1 }
            else if c == '^' { startY-=1}
            else if c == '<' { startX-=1 }
            else if c == 'v' { startY+=1 }
        }
        return startY == self.endPoint.0 && startX == self.endPoint.1
    }
}


#[cfg(test)]
mod tests_monstrous_maze {
    use super::*;

    #[test]
    fn is_monstrous_maze_name() {
        assert_eq!(MonstrousMaze::name(), String::from("monstrousMaze"));
    }

    #[test]
    fn is_monstrous_maze_new() {
        let new_maze = MonstrousMaze::new(MonstrousMazeInput{endurance: 2, grid: "│Y M X│".to_string()});
        assert_eq!(new_maze.input.endurance, 2);
        assert_eq!(new_maze.input.grid, "│Y M X│");
    }

    #[test]
    fn is_monstrous_maze_in_grid() {
        let subgrid = "│Y M X│".lines().map(str::to_string).collect();
        let grid = Grid{grid: subgrid ,start:(0,1) ,end: (0,5) };
        let is_in_grid = MonstrousMaze::is_coordinates_in_grid( (0,2), &grid);
        assert_eq!(is_in_grid, true);
    }

    #[test]
    fn is_monstrous_maze_monster() {
        let subgrid = "│Y M X│".lines().map(str::to_string).collect();
        let grid = Grid{grid: subgrid ,start:(0,1) ,end: (0,5) };
        let is_monster_coordinate = MonstrousMaze::is_coordinates_monster( (0,3), &grid);
        assert_eq!(is_monster_coordinate, true);
    }

    #[test]
    fn is_monstrous_maze_verify() {
        let new_maze = MonstrousMaze::new(MonstrousMazeInput{endurance: 2, grid: "│Y M X│".to_string()});
        let output = MonstrousMazeOutput{path: ">>>>".to_string()};
        assert_eq!(MonstrousMaze::verify(&new_maze,&output), true);
    }
}
