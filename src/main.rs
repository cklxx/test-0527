extern crate markup5ever_rcdom as rcdom;
extern crate xml5ever;

use std::collections::HashMap;
use std::default::Default;
use std::string::String;

use colors_transform::{Color as VColor, Rgb};
use macroquad::miniquad::date::now;
use rcdom::{Handle, NodeData, RcDom};
use xml5ever::driver::parse_document;
use xml5ever::local_name;
use xml5ever::tendril::TendrilSink;
// use html5ever::parse_document;
// use html5ever::tendril::TendrilSink;
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use stretch::geometry::{Point, Size};
use stretch::node::Node;
use stretch::style::*;

// This is not proper HTML serialization, of course.

#[macroquad::main("BasicShapes")]
async fn main() {
    let (mut layout, mut noder) = mock_layout("test/foo.html").await;
    let mut frame = 0.;
    let start = now();
    loop {
        if frame == 60. {
            (layout, noder) = mock_layout("test/bar.html").await;
        }
        frame += 1.;
        layout
            .render_layout(&noder, Point { x: 0.0, y: 0.0 })
            .unwrap();

        let fps = frame / (now() - start);

        draw_text_ex(
            &format!("FPS: {}", &fps.to_string()).to_string(),
            500.,
            500.,
            TextParams {
                font_size: 15,
                // font: layout.font,
                ..Default::default()
            },
        );
        next_frame().await
    }
}
#[derive(Debug, Default)]
struct VAttributes {
    background_color: macroquad::prelude::Color,
    text: String,
}
struct Layout {
    stretch: stretch::Stretch,
    store: HashMap<Node, VAttributes>,
    // font: Font,
}

impl Layout {
    fn new() -> Layout {
        let store = HashMap::new();
        Layout {
            stretch: stretch::node::Stretch::new(),
            store,
            // font,
        }
    }
    fn new_node(&mut self, style: Style, children: Vec<Node>) -> Result<Node, stretch::Error> {
        self.stretch.new_node(style, children)
    }
    fn get_loc(root: &stretch::result::Layout, loc: Point<f32>) -> (f32, f32) {
        (root.location.x + loc.x, root.location.y + loc.y)
    }

    fn render_layout(
        &self,
        root: &stretch::node::Node,
        location: Point<f32>,
    ) -> Result<(), stretch::Error> {
        let root_layout = self.stretch.layout(*root).unwrap();

        let mut rng = thread_rng();
        let mut color = Color::new(255.0, 255.0, 255.0, 0.0);
        let mut text = String::new();
        if let Some(attr) = self.store.get(root) {
            color = attr.background_color;
            text = attr.text.clone();
        }
        let fcolor = Color::new(
            rng.gen::<f32>() % 255.0,
            rng.gen::<f32>() % 255.0,
            rng.gen::<f32>() % 255.0,
            1.0,
        );

        let (x, y) = Layout::get_loc(root_layout, location);
        if root_layout.size.height != 0.0 && root_layout.size.width != 0.0 {
            draw_rectangle(x, y, root_layout.size.width, root_layout.size.height, color);
        }
        if text != "" {
            draw_text_ex(
                &text,
                x,
                y + 20.,
                TextParams {
                    font_size: 20,
                    // font: self.font,
                    color: fcolor,
                    ..Default::default()
                },
            );
        }

        if let Ok(nodes) = self.stretch.children(*root) {
            for node in nodes {
                self.render_layout(&node, Point { x, y }).unwrap();
            }
        }
        Ok(())
    }

    fn walk(&mut self, prefix: &str, handle: &Handle, root: Node) {
        let node = handle;
        let vnode = self
            .stretch
            .new_node(
                Style {
                    ..Default::default()
                },
                vec![],
            )
            .unwrap();

        match node.data {
            NodeData::Document => (),

            NodeData::Text { ref contents } => {
                self.store.insert(
                    vnode,
                    VAttributes {
                        text: trim_text(contents.borrow().to_string()),
                        ..Default::default()
                    },
                );
            }

            NodeData::Element { ref attrs, .. } => {
                for attr in attrs.borrow().iter() {
                    match attr.name.local {
                        local_name!("style") => {
                            // let mut style = StyleAttribute::parse(&attr.value).unwrap();
                            let style_strs: Vec<String> = format!("{}", &attr.value)
                                .split(";")
                                .map(|s| s.parse::<String>().unwrap())
                                .collect::<Vec<String>>();

                            let (style, other_style) = parse_style(style_strs);
                            self.store.insert(vnode, other_style);
                            self.stretch.set_style(vnode, style).unwrap();
                        }
                        _ => (),
                    }
                }
            }

            _ => {}
        }
        self.stretch.add_child(root, vnode).unwrap();
        let new_indent = {
            let mut temp = String::new();
            temp.push_str(prefix);
            temp.push_str("    ");
            temp
        };

        for child in node
            .children
            .borrow()
            .iter()
            .filter(|child| match &child.data {
                NodeData::Text { contents } => {
                    let mut result = true;
                    let mut text_temp: String = contents.borrow().to_string();
                    text_temp = text_temp.trim().to_string();
                    if text_temp == "" {
                        result = false;
                    }
                    result
                }
                NodeData::Element { .. } => true,
                _ => false,
            })
        {
            self.walk(&new_indent, child, vnode);
        }
    }
}

async fn mock_layout(path: &str) -> (Layout, stretch::node::Node) {
    // let f = File::open(path).unwrap();
    // let font = load_ttf_font("test/杨任东竹石体-Light.ttf").await.unwrap();
    // let mut f = BufReader::new(f);

    let html = r#"<body>
	<div
		id="box"
		style="width: 400; height: 333; background-color: rgb(87, 214, 73);display: flex;flex-direction: column;"
	>
		box
		<div
			id="overview"
			style="width: 100; height: 333; background-color: rgb(172, 31, 31)"
		>
			搜索
		</div>
		<div
			id="overview"
			style="width: 100; height: 333; background-color: rgb(20, 133, 203)"
		>
			搜索界面
		</div>
		<div
			id="overview"
			style="width: 200; height: 333; background-color: rgb(172, 31, 31)"
		>
			<div
				id="overview"
				style="
					width: 100;
					height: 333;
					background-color: rgb(40, 82, 109);
				"
			>
				hello world
			</div>
			<div
				id="overview"
				style="
					width: 10;
					height: 333;
					background-color: rgb(12, 117, 182);
				"
			>
				hello world2
			</div>
		</div>
	</div>
</body>
"#;
    // To parse XML into a tree form, we need a TreeSink
    // luckily xml5ever comes with a static RC backed tree represetation.
    let dom: RcDom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    // Execute our visualizer on RcDom

    let mut layout = Layout::new();
    let (w, h) = (screen_width(), screen_height());
    let root = layout
        .new_node(
            Style {
                size: Size {
                    width: Dimension::Points(w),
                    height: Dimension::Points(h),
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();

    layout.walk("", &dom.document, root);

    layout
        .stretch
        .compute_layout(root, Size::undefined())
        .unwrap();
    (layout, root)
}

fn trim_text(text: String) -> String {
    let x: &[_] = &['\n', '\t'];
    text.trim_matches(x).to_string()
}

fn parse_style(style_strs: Vec<String>) -> (Style, VAttributes) {
    let mut style = Style {
        display: Display::Flex,
        ..Default::default()
    };
    let mut other_style = VAttributes {
        ..Default::default()
    };
    for style_item in style_strs.iter() {
        let s = style_item
            .split(":")
            .map(|s| s.trim().parse::<String>().unwrap())
            .collect::<Vec<String>>();
        match s[0].as_str() {
            "width" => {
                style.size.width = Dimension::Points(s[1].trim().parse::<f32>().unwrap());
            }
            "height" => {
                style.size.height = Dimension::Points(s[1].trim().parse::<f32>().unwrap());
            }
            "display" => {
                style.display = parse_style_display(s[1].trim());
            }
            "flex-direction" => {
                style.flex_direction = parse_flex_direction(s[1].trim());
            }
            "background-color" => {
                let rgb = s[1].parse::<Rgb>().unwrap();
                other_style.background_color = Color::new(
                    rgb.get_red() / 255.0,
                    rgb.get_green() / 255.0,
                    rgb.get_blue() / 255.0,
                    1.0,
                )
            }
            _ => (),
        }
    }
    (style, other_style)
}

fn parse_style_display(display: &str) -> Display {
    match display {
        "flex" => Display::Flex,
        _ => Display::None,
    }
}

fn parse_flex_direction(flex_direction: &str) -> FlexDirection {
    match flex_direction {
        "column" => FlexDirection::Column,
        "column-reverse" => FlexDirection::ColumnReverse,
        "row-reverse" => FlexDirection::RowReverse,
        "row" | _ => FlexDirection::Row,
    }
}
