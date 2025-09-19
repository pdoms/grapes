use crate::{linal::vertx2::VX2, objects::Vertices, vx2};

fn triple_prod(a: &VX2, b: &VX2, c: &VX2) -> VX2 {
    let cross_ab = a.x * b.y - a.y * b.x;
    vx2!(-c.y * cross_ab, c.x * cross_ab)
}

fn perpendicular_toward_origin(edge: &VX2, ao: &VX2) -> VX2 {
    let perp = triple_prod(edge, ao, edge);
    if perp.x.abs() < f32::EPSILON && perp.y.abs() < f32::EPSILON {
        let mut alt = vx2!(-edge.y, edge.x);
        if alt.dot(ao) < 0.0 {
            alt.x = -alt.x;
            alt.y = -alt.y;
        }
        return alt;
    }
    return perp;
}

fn average_point(vertices: &[VX2]) -> VX2 {
    let mut avg = vx2!(0.0);
    assert!(vertices.len() > 0, "No division by zero");
    for vert in vertices {
        avg.x += vert.x;
        avg.y += vert.y;
    }
    avg /= vertices.len() as f32;
    avg
}

fn furthest_vertex(vertices: &[VX2], dir: &VX2) -> usize {
    let mut max_prod = f32::NEG_INFINITY;
    let mut index = 0;
    for i in 0..vertices.len() {
        let prod = dir.dot(&vertices[i]);
        if prod > max_prod {
            max_prod = prod;
            index = i;
        }
    }
    return index;
}

fn support(verts1: &[VX2], verts2: &[VX2], dir: &VX2) -> VX2 {
    // get furthest point of first body along an arbitrary direction
    let i = furthest_vertex(verts1, dir);
    // get furthest point of second body along the opposite direction
    let j = furthest_vertex(verts2, &-dir);
    // subtract (Minkowski sum) the two points to see if bodies 'overlap'
    verts1[i] - &verts2[j]
}

fn handle_simplex(simplex: &mut Vec<VX2>, dir: &mut VX2) -> bool {
    if simplex.len() == 2 {
        let a = simplex[1];
        let b = simplex[0];
        let ao = -a;
        let ab = b - &a;

        let dir_v = perpendicular_toward_origin(&ab, &ao);
        dir.x = dir_v.x;
        dir.y = dir_v.y;
        false
    } else if simplex.len() == 3 {
        let a = simplex[2];
        let b = simplex[1];
        let c = simplex[0];
        let ao = -a;
        let ab = b - &a;
        let ac = c - &a;

        let perp_ab = triple_prod(&ac, &ab, &ab);
        let perp_ac = triple_prod(&ab, &ac, &ac);

        if perp_ab.dot(&ao) > 0.0 {
            simplex.remove(0);
            dir.x = perp_ab.x;
            dir.y = perp_ab.y;
            return false;
        }

        if perp_ac.dot(&ao) > 0.0 {
            simplex.remove(1);
            dir.x = perp_ac.x;
            dir.y = perp_ac.y;
            return false;
        }
        true
    } else {
        unreachable!()
    }
}

pub fn gjk(verts1: &[VX2], verts2: &[VX2]) -> bool {
    let mut dir = average_point(verts1) - &average_point(verts2);
    let mut simplex = Vec::with_capacity(3);

    if dir.x == 0.0 && dir.y == 0.0 {
        dir = vx2!(1.0, 0.0);
    }
    simplex.push(support(verts1, verts2, &dir));
    dir = -simplex[0];
    let mut new_point: VX2;

    loop {
        new_point = support(verts1, verts2, &dir);
        if new_point.dot(&dir) <= 0.0 {
            return false;
        }
        simplex.push(new_point);
        if handle_simplex(&mut simplex, &mut dir) {
            return true;
        }
    }
}
