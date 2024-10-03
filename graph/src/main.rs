use deno_graph::source::MemoryLoader;
use deno_graph::source::Source;
use deno_graph::BuildFastCheckTypeGraphOptions;
use deno_graph::GraphKind;
use deno_graph::ModuleGraph;
use deno_graph::ModuleSpecifier;
use deno_graph::WorkspaceFastCheckOption;
use deno_graph::WorkspaceMember;
use futures::executor::block_on;

fn main() {
    let loader = MemoryLoader::new(
        vec![(
            "file:///test.ts",
            Source::Module {
                specifier: "file:///test.ts",
                maybe_headers: None,
                content: "
                        export class Test {
                            test(): string {
                                return 'test';
                            }
                        }
                    ",
            },
        )],
        Vec::new(),
    );
    let roots = vec![ModuleSpecifier::parse("file:///test.ts").unwrap()];

    let mut exports = indexmap::IndexMap::new();
    exports.insert(".".to_string(), "./test.ts".to_string());

    let workspace_members = vec![WorkspaceMember {
        base: url::Url::parse("file:///").unwrap(),
        exports: exports.clone(),
        name: "@foo/bar".to_string(),
        version: Some(deno_semver::Version::parse_standard("1.0.0").unwrap()),
    }];

    let future = async move {
        let mut graph = ModuleGraph::new(GraphKind::All);
        graph.build(roots, &loader, Default::default()).await;
        let options = BuildFastCheckTypeGraphOptions {
            fast_check_cache: None,
            fast_check_dts: true,
            workspace_fast_check: WorkspaceFastCheckOption::Enabled(&workspace_members),
            ..Default::default()
        };
        graph.build_fast_check_type_graph(options);
        graph.valid().unwrap();

        graph
    };

    let graph = block_on(future);

    for module in graph.modules() {
        println!("{:#?}", module.js().unwrap().fast_check);
    }
}
