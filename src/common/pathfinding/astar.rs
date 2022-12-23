use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
    marker::PhantomData,
};

use super::*;
use thiserror::Error;

#[derive(Debug)]
struct PartialPath<'a, W: World<'a>, S: Score> {
    world: &'a W,
    positions: Vec<W::Index>,
    start_distance: S,
    hueristic_distance: S,
}

impl<'a, W: World<'a>, S: Score> Clone for PartialPath<'a, W, S> {
    fn clone(&self) -> Self {
        PartialPath {
            world: self.world,
            positions: self.positions.clone(),
            start_distance: self.start_distance.clone(),
            hueristic_distance: self.hueristic_distance.clone(),
        }
    }
}
impl<'a, W: World<'a>, S: Score> PartialPath<'a, W, S> {
    fn append(&mut self, point: W::Index, distance: S) {
        self.positions.push(point);
        self.start_distance += distance;
    }
}

impl<'a, W: World<'a>, S: Score> PartialOrd for PartialPath<'a, W, S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut a = self.start_distance.clone();
        a += self.hueristic_distance.clone();
        let mut b = other.start_distance.clone();
        b += other.hueristic_distance.clone();
        a.partial_cmp(&b)
    }
}

impl<'a, W: World<'a>, S: Score> Ord for PartialPath<'a, W, S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut a = self.start_distance.clone();
        a += self.hueristic_distance.clone();
        let mut b = other.start_distance.clone();
        b += other.hueristic_distance.clone();
        a.cmp(&b)
    }
}

impl<'a, W: World<'a>, S: Score> PartialEq for PartialPath<'a, W, S> {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions
    }
}

impl<'a, W: World<'a>, S: Score> Eq for PartialPath<'a, W, S> {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("No path is possible")]
    NoPossiblePath,
}

pub struct Algorithm<'a, W: World<'a>, A: Agent<'a, W>, H: Fn(&W, &W::Index, &W::Index) -> A::Score>
{
    hueristic: H,
    _life: PhantomData<&'a ()>,
    _w: PhantomData<W>,
    _a: PhantomData<A>,
}

impl<'a, W: World<'a>, A: Agent<'a, W>, H: Fn(&W, &W::Index, &W::Index) -> A::Score>
    Algorithm<'a, W, A, H>
{
    pub fn new(hueristic: H) -> Self {
        Self {
            hueristic,
            _life: Default::default(),
            _w: Default::default(),
            _a: Default::default(),
        }
    }
}

impl<'a, W, A, H> super::Algorithm<'a, W, A> for Algorithm<'a, W, A, H>
where
    W: World<'a>,
    A: Agent<'a, W>,
    H: Fn(&W, &W::Index, &W::Index) -> A::Score,
{
    type Error = Error;

    fn get_path(
        &self,
        world: &'a W,
        agent: &A,
        start: W::Index,
        target: W::Index,
    ) -> Result<Path<'a, W>, Self::Error> {
        let mut paths = BinaryHeap::new();
        let mut visited = HashSet::new();
        visited.insert(start.clone());

        paths.push(Reverse(PartialPath {
            world,
            positions: vec![start.clone()],
            start_distance: A::Score::default(),
            hueristic_distance: (self.hueristic)(world, &start, &target),
        }));
        loop {
            let shortest = paths.pop().ok_or(Error::NoPossiblePath)?;
            let shortest = shortest.0;

            let head = shortest.positions.last().unwrap();

            for neighbor in world.get_neighbors(head) {
                if visited.contains(&neighbor) {
                    continue;
                }

                let dist = match agent.get_cost(world, &head, &neighbor) {
                    Some(x) => x,
                    None => continue,
                };

                let mut path = shortest.clone();
                path.append(neighbor.clone(), dist);

                if neighbor == target {
                    let path = super::Path {
                        world,
                        positions: path.positions,
                    };
                    return Ok(path);
                }

                let hue = (self.hueristic)(world, &neighbor, &target);
                path.hueristic_distance = hue;
                paths.push(Reverse(path));

                visited.insert(neighbor);
            }
        }
    }
}
