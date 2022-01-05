pub mod css;
pub mod cssom;
pub mod dom;
pub mod html;
pub mod layout;
pub mod painting;
pub mod parser;
pub mod render;
pub mod window;

fn main() {
    let mut window = window::Window::new();

    let dom = html::Parser::parse(
        "
        <html>
            <head>
                <title>Hello World!</title>
            </head>
            <body>
                <main>
                    <div class=\"first\"></div>
                    <div class=\"second\">
                        <div class=\"inner\"></div>
                    </div>
                </main>
            </body>
        </html>
    ",
    );

    let styles = css::Parser::parse(
        "
        .first {
            width: 33%;
            height: 50px;
            background-color: #f00;
        }

        .second {
            width: 66%;
            height: 200px;
            margin-top: 10px;
            background-color: #00f;
        }

        .inner {
            width: 50%;
            height: 50%;
            background-color: #0f0;
        }
    ",
    );

    let viewport = layout::Dimensions::new(640, 480);
    let anchor = layout::Point::new(0, 0);

    let render_node = render::RenderNode::from(&dom[0], &styles);
    let layout_node = layout::LayoutNode::from(&render_node, &viewport, &anchor);

    window.paint_node(&layout_node.children[1].children[0].children[0]);
    window.paint_node(&layout_node.children[1].children[0].children[1]);

    while window.is_open() {
        window.update();
    }
}
