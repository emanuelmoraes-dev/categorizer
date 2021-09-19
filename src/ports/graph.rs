pub trait Item<'a, I> {
    fn get_id(&self) -> &I;
}

pub trait Graph<'a, V, E, I> where
    V: Item<'a, I>,
    E: Item<'a, I>
{
    fn add_vertex(&self, v: V);
    fn remove_vertex_by(&self, v: &V) -> (V, Vec<E>);
    fn remove_vertex_at(&self, idv: &I) -> (V, Vec<E>);
    fn get_vertex_at(&self, idv: &I) -> &V;

    fn get_vertexes(&self) -> Vec<&V>;
    fn get_vertexes_by(&self, e1: &E, e2: &E) -> (&V, &V);
    fn get_vertexes_at(&self, ide1: &I, ide2: &I) -> (&V, &V);

    fn add_edge(&self, e: E);
    fn remove_edge_by(&self, e: &E) -> E;
    fn remove_edge_at(&self, ide: &I) -> E;
    fn get_edge_at(&self, ide: &I) -> &E;

    fn get_edges(&self) -> Vec<&E>;
    fn get_edges_by(&self, v1: &V, v2: &V) -> Vec<&E>;
    fn get_edges_at(&self, idv1: &I, idv2: &I) -> Vec<&E>;

    fn link(&self, v1: V, v2: V, e: E);
    fn link_by(&self, v1: &V, v2: &V);
    fn link_at(&self, idv1: &I, idv2: &I, ide: &I);
}
