use crate::ports::graph;

pub enum Origin {
    Vertex,
    Edge
}

pub enum Source<'a, V, E, I> {
    Item {
        value: &'a dyn graph::Item<'a, I>,
        graph: &'a dyn graph::Graph<'a, V, E, I>,
        origin: Origin
    },
    Id {
        value: &'a I,
        graph: &'a dyn graph::Graph<'a, V, E, I>,
        origin: Origin
    }
}

pub trait Element<'a, V, E, I> {
    fn from(source: Source<'a, V, E, I>);

    fn get_source(&self) -> &Source<'a, V, E, I>;
    fn get_def(&self) -> dyn graph::Graph<'a, V, E, I>;
    fn get_def_item(&self) -> &dyn graph::Item<'a, I>;
}
