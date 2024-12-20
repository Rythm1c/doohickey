use gl;
mod src;
mod demo;

fn main() {
    use demo::Demo;
    let mut app = Demo::new();
    app.run();
}
