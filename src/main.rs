pub mod css;
pub mod cssom;
pub mod dom;
pub mod html;
pub mod layout_tree;
pub mod painting;
pub mod parser;
pub mod render_tree;
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
                    <h1>Hello World!</h1>
                    <p>Lorem Ipsum dolor sit amet</p>
                </main>
            </body>
        </html>
    ",
    );

    let styles = css::Parser::parse(
        "
        h1 {
            height: 50px;
            background-color: #f00;
        }

        p {
            height: 200px;
            margin-top: 10px;
            background-color: #00f;
        }
    ",
    );

    let viewport = layout_tree::Dimensions::new(640, 480);
    let anchor = layout_tree::Point::new(0, 0);

    let render_node = render_tree::RenderNode::from(&dom[0], &styles);
    let layout_node = layout_tree::LayoutNode::from(&render_node, &viewport, &anchor);

    window.paint_node(&layout_node.children[1].children[0].children[0]);
    window.paint_node(&layout_node.children[1].children[0].children[1]);

    while window.is_open() {
        window.update();
    }
}
