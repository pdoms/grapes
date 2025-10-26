use std::f32;

use crate::{context, linal::vertx2::VX2, utils::lerp_vx2, vx2};

const EPA_TOL: f32 = 1e-6;
const EPA_MAX_ITERS: usize = 64;

#[derive(Clone, Copy, Debug)]
pub struct EpaVertex {
    pub v: VX2,
    pub a: VX2,
    pub b: VX2,
}

#[derive(Debug, Clone)]
pub struct EpaResult {
    /// unit vector pointing from A to B
    pub normal: VX2,
    /// penetration depth (positive)
    pub depth: f32,
    /// single contact on A (world)
    pub contact_a: VX2,
    /// single contact on B (world)
    pub contact_b: VX2,
    /// optional 1-2 contact pairs (A,B)
    pub contacts: Vec<(VX2, VX2)>,
}

pub fn make_vertex<F, G>(support_a: F, support_b: G, dir: &VX2) -> EpaVertex
where
    F: Fn(&VX2) -> VX2,
    G: Fn(&VX2) -> VX2,
{
    let sa = support_a(dir);
    let sb = support_b(&-*dir);
    EpaVertex {
        v: sa - &sb,
        a: sa,
        b: sb,
    }
}

fn edge_closest(poly: &[EpaVertex]) -> (VX2, f32, usize, usize) {
    let mut best_n = vx2!(0.0, 0.0);
    let mut best_dist = f32::INFINITY;
    let mut best_i = 0usize;
    let mut best_j = 1usize;

    let n = poly.len();
    for i in 0..n {
        let j = (i + 1) % n;
        let vi = poly[i].v;
        let edge = poly[j].v - &vi;

        let mut normal = vx2!(-edge.y, edge.x);

        let len = normal.length();
        if len.abs() < f32::EPSILON {
            continue;
        }
        normal /= len;

        let dist = normal.dot(&vi);
        let (normal, dist) = if dist < 0.0 {
            (-normal, -dist)
        } else {
            (normal, dist)
        };
        if dist < best_dist {
            best_dist = dist;
            best_n = normal;
            best_i = i;
            best_j = j;
        }
    }
    (best_n, best_dist, best_i, best_j)
}

pub fn project_origin_on_segment(v0: VX2, v1: VX2) -> f32 {
    let seg = v1 - &v0;
    let denom = seg.dot(&seg);
    if denom.abs() < f32::EPSILON {
        return 0.0;
    }
    let t = (-v0).dot(&seg) / &denom;
    t.clamp(0.0, 1.0)
}

fn centroid(poly: &[EpaVertex]) -> VX2 {
    let mut c = vx2!(0.0, 0.0);
    for p in poly {
        c += &p.v
    }
    let n = poly.len() as f32;
    c /= n;
    c
}

fn perpendicular(v: VX2) -> VX2 {
    vx2!(-v.y, v.x)
}

pub fn epa<F, G>(
    mut initial_simplex: Vec<EpaVertex>,
    support_a: F,
    support_b: G,
) -> Option<EpaResult>
where
    F: Fn(&VX2) -> VX2,
    G: Fn(&VX2) -> VX2,
{
    if initial_simplex.len() < 3 {
        return None;
    }

    let center = centroid(&initial_simplex);
    initial_simplex.sort_by(|p, q| {
        let ap = (p.v - &center).y.atan2((p.v - &center).x);
        let aq = (p.v - &center).y.atan2((q.v - &center).x);
        ap.partial_cmp(&aq).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut poly = initial_simplex;

    for _ in 0..EPA_MAX_ITERS {
        let (mut normal, best_dist, i, j) = {
            let mut best_n = vx2!(0.0, 0.0);
            let mut best_d = f32::INFINITY;
            let mut bi = 0usize;
            let mut bj = 1usize;
            let n = poly.len();
            for ii in 0..n {
                let jj = (ii + 1) % n;
                let vi = poly[ii].v;
                let vj = poly[jj].v;
                let edge = vj - &vi;
                let mut nrm = perpendicular(edge);
                let len = nrm.length();
                if len.abs() < f32::EPSILON {
                    continue;
                }
                nrm /= len;
                let signed = nrm.dot(&vi);
                let (nrm, d) = if signed < 0.0 {
                    (-nrm, -signed)
                } else {
                    (nrm, signed)
                };
                if d < best_d {
                    best_d = d;
                    best_n = nrm;
                    bi = ii;
                    bj = jj;
                }
            }
            (best_n, best_d, bi, bj)
        };

        let new_v = {
            let sa = support_a(&normal);
            let sb = support_b(&-normal);
            EpaVertex {
                v: sa - &sb,
                a: sa,
                b: sb,
            }
        };

        let support_dist = normal.dot(&new_v.v);

        if (support_dist - best_dist) < EPA_TOL {
            let v0 = poly[i];
            let v1 = poly[j];

            let seg = v1.v - &v0.v;
            let denom = seg.dot(&seg);
            let t = if denom.abs() < f32::EPSILON {
                0.0
            } else {
                (-v0.v).dot(&seg) / denom
            }
            .clamp(0.0, 1.0);

            let contact_a_pair = lerp_vx2(&v0.a, &v1.a, t);
            let contact_b_pair = lerp_vx2(&v0.b, &v1.b, t);
            let mut n = normal;
            let mut depth = n.dot(&(contact_b_pair - &contact_a_pair));
            if depth < 0.0 {
                n = -n;
                depth = -depth;
            }

            let contact_a = contact_a_pair;
            let contact_b = contact_b_pair;

            let mut contacts = Vec::new();

            contacts.push((contact_a_pair, contact_b_pair));
            if t > 1e-4 && t < 1.0 - 1e-4 {
                if (new_v.v - &v0.v).length() > 1e-6 && (new_v.v - &v1.v).length() > 1e-6 {
                    contacts.push((new_v.a, new_v.b))
                }
            }
            return Some(EpaResult {
                normal: n,
                depth,
                contact_a,
                contact_b,
                contacts,
            });
        }

        poly.insert(j, new_v);
    }

    None
}
