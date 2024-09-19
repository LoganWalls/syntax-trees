pub mod parser;
pub mod tree;

use leptos::*;

use self::parser::parse_syntax_tree;

use web_sys::HtmlDivElement;

const DEFAULT_SRC: &str = r#"
[S 
  [NP [N Ash] ]
  [VP
    [V caught]
    [NP
      [Det the]
      [NP
        [N Mew]
        [PP
          [P with]
          [NP
            [Det the]
            [NP [N Pokeball] ]
          ]
        ]
      ]
    ]
  ]
]"#;

#[component]
pub fn CodeEditor(src: RwSignal<String>) -> impl IntoView {
    // TODO: copy button
    // TODO: syntax highlighting
    let src_html = move || src.get();

    view! {
        <div class="relative flex basis-1/3 grow h-fit">
            <pre class="relative min-w-[45em] max-w-[60em] min-h-40 z-10 px-4 py-2 overflow-x-scroll  bg-slate-700 dark:bg-slate-900 rounded-md">
                <div
                    contenteditable="true"
                    spellcheck="false"

                    class="px-4 py-2 absolute size-full top-0 left-0 bg-transparent text-transparent cursor-text whitespace-pre focus:outline-none overflow-visible caret-slate-500 dark:caret-slate-200"
                    on:input=move |ev| { src.set(event_target::<HtmlDivElement>(&ev).inner_text()) }
                >
                    {src.get_untracked()}
                </div>
                <code class="select-none size-full" inner_html=src_html></code>
            </pre>
        </div>
    }
}

fn node_coord_to_svg(node: &tree::Node) -> (f32, f32) {
    let x_pad = 50.0;
    let y_pad = 50.0;
    let x = node.x * 120.0 + x_pad;
    let y = node.y * 100.0 + y_pad;
    (x, y)
}

#[component]
pub fn SyntaxTreeNodeRender(node: tree::Node) -> impl IntoView {
    let (x, y) = node_coord_to_svg(&node);
    let inner = match *node.kind {
        tree::NodeKind::Leaf { label } => {
            let label_y = y + 40.0;
            view! {
                <text x=x y=label_y text-anchor="middle" class="fill-slate-900 dark:fill-slate-400">
                    {label}
                </text>
            }
            .into_view()
        }
        tree::NodeKind::Subtree { left, right } => {
            let offset_y = 10.0;
            let child_offset_y = -25.0;
            let (left_x, left_y) = node_coord_to_svg(&left);
            if let Some(right) = right {
                let offset_x = 10.0;
                let (right_x, right_y) = node_coord_to_svg(&right);
                view! {
                    <line
                        x1=x + offset_x
                        x2=right_x - offset_x
                        y1=y + offset_y
                        y2=right_y + child_offset_y
                        class="stroke-slate-900"
                    />
                    {__SyntaxTreeNodeRender(right)}
                    <line
                        x1=x - offset_x
                        x2=left_x + offset_x
                        y1=y + offset_y
                        y2=left_y + child_offset_y
                        class="stroke-slate-900"
                    />
                    {__SyntaxTreeNodeRender(left)}
                }
            } else {
                view! {
                    <line
                        x1=x
                        x2=left_x
                        y1=y + offset_y
                        y2=left_y + child_offset_y
                        class="stroke-slate-900"
                    />
                    {__SyntaxTreeNodeRender(left)}
                }
            }
            .into_view()
        }
    };

    view! {
        <text x=x y=y text-anchor="middle" class="fill-sky-500">
            {node.category.to_string()}
        </text>
        {inner}
    }
}

#[component]
pub fn SyntaxTreeRender(src: RwSignal<String>) -> impl IntoView {
    let tree_root = move || {
        let (_, tree) = parse_syntax_tree(&src.get()).unwrap();
        view! { <SyntaxTreeNodeRender node=tree.root /> }
    };
    view! {
        <div class="h-fit max-h-full flex basis-2/3 shrink">
            <svg class="w-full h-auto" viewBox="0 0 1000 1000">
                {tree_root}
                Sorry but this browser does not support inline SVG.
            </svg>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let src = create_rw_signal(DEFAULT_SRC.to_string());
    view! {
        <div class="size-full flex flex-row place-items-top">
            <SyntaxTreeRender src=src />
            <CodeEditor src=src />
        </div>
    }
}
