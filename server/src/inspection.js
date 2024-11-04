export class CloudstateInspectionCS {
    static id = "inspection";

    listRoots() {
        return Deno.core.ops.op_cloudstate_list_roots().filter((root) =>
            root !== "inspection"
        );
    }

    run(script) {
        // internal
    }

    status() {
        return {
            status: "ok",
        };
    }
}
