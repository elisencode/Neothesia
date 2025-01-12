mod iced_menu;

mod icons;

use std::time::Duration;

use iced_menu::AppUi;
use neothesia_core::render::BgPipeline;

use wgpu_jumpstart::{TransformUniform, Uniform};
use winit::event::WindowEvent;

use crate::{
    context::Context,
    iced_utils::{
        iced_conversion,
        iced_state::{self, Program},
    },
    scene::Scene,
};

type Renderer = iced_wgpu::Renderer;

pub struct MenuScene {
    bg_pipeline: BgPipeline,
    iced_state: iced_state::State<AppUi>,

    context: std::task::Context<'static>,
    futures: Vec<futures::future::BoxFuture<'static, iced_menu::Message>>,
}

impl MenuScene {
    pub fn new(ctx: &mut Context) -> Self {
        let menu = AppUi::new(ctx);
        let iced_state =
            iced_state::State::new(menu, ctx.iced_manager.viewport.logical_size(), ctx);

        Self {
            bg_pipeline: BgPipeline::new(&ctx.gpu),
            iced_state,

            context: std::task::Context::from_waker(futures::task::noop_waker_ref()),
            futures: Vec::new(),
        }
    }
}

impl Scene for MenuScene {
    fn update(&mut self, ctx: &mut Context, delta: Duration) {
        self.bg_pipeline.update_time(&mut ctx.gpu, delta);
        self.iced_state.tick(ctx);

        self.futures
            .retain_mut(|f| match f.as_mut().poll(&mut self.context) {
                std::task::Poll::Ready(msg) => {
                    self.iced_state.queue_message(msg);
                    false
                }
                std::task::Poll::Pending => true,
            });

        // Let's skip for now, as we need the tick update every frame anyway
        // if self.iced_state.is_queue_empty() {
        //     return;
        // }

        if let Some(command) = self.iced_state.update(ctx) {
            for a in command.actions() {
                match a {
                    iced_runtime::command::Action::Future(f) => {
                        self.futures.push(f);
                    }
                    _ => {}
                }
            }
        }
    }

    fn render<'pass>(
        &'pass mut self,
        _transform: &'pass Uniform<TransformUniform>,
        rpass: &mut wgpu::RenderPass<'pass>,
    ) {
        self.bg_pipeline.render(rpass);
    }

    fn window_event(&mut self, ctx: &mut Context, event: &WindowEvent) {
        use winit::keyboard::ModifiersState;

        let modifiers = ModifiersState::default();

        if let Some(event) = iced_conversion::window_event(
            event.clone(),
            ctx.iced_manager.viewport.scale_factor(),
            modifiers,
        ) {
            self.iced_state.queue_event(event.clone());

            match &event {
                iced_core::event::Event::Mouse(event) => {
                    if let Some(msg) = self.iced_state.program().mouse_input(event, ctx) {
                        self.iced_state.queue_message(msg);
                    }
                }
                iced_core::event::Event::Keyboard(event) => {
                    if let Some(msg) = self.iced_state.program().keyboard_input(event, ctx) {
                        self.iced_state.queue_message(msg);
                    }
                }
                _ => {}
            }
        }
    }
}
