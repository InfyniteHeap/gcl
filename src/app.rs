use windows_reactor::*;

pub(crate) struct Launcher {}

impl Component for Launcher {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));

        StackPanel::new().children((TitleBar::new()
            .title("Grid Craft Launcher")
            .preferred_height(WindowTitleBarHeight::Tall),))
    }
}
