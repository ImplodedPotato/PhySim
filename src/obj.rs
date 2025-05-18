use crate::ball::Ball;

pub trait Collision {
    fn collision_with_ball(&mut self, ball: &mut Ball);
    fn collision_with_rect<T>(rect: &mut T);
}

pub trait PhyObj {
    unsafe fn update_gravity(&mut self, dt: f32);
    unsafe fn update_movement(&mut self, dt: f32);
    unsafe fn update_clamp(&mut self, screen: raylib::Vector2);
    unsafe fn update(&mut self, screen: raylib::Vector2, dt: f32);
}
