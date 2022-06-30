//////////////////////////////////////////////////
// Using

pub mod physic;
pub(crate) use physic::PhysicSystem;

pub mod interaction;
pub(crate) use interaction::InteractionSystem;

pub mod morph;
pub(crate) use morph::MorphSystem;

pub mod renderer;
pub(crate) use renderer::RenderSystem;

// pub mod role;
// pub(crate) use role::RoleSystem;

// pub mod lifetime;
// pub(crate) use lifetime::LifetimeSystem;

// pub mod particle;
// pub(crate) use particle::ParticleSystem;

// pub mod animation;
// pub(crate) use animation::AnimationSystem;

// pub mod camera;
// pub(crate) use camera::CameraSystem;
